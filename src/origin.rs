use prettytable::{Cell, Row, Table};
use reqwest::blocking::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Debug)]
pub struct OriginApi {
    pub(crate) remid: String,
    pub(crate) sid: String,
    pub(crate) auth_token: String,
}

impl OriginApi {
    // This will fail when my creds expire
    pub fn from_hardcoded() -> Self {
        Self {
            sid: "Uzk0NXRKSmhoclVKWmFGOTJ5aFBIeHd5R25ZaU90MG05R1JSck5zaWNxSXBXMUlCOHNMR1RFVDRGTzMyeQ.NMnvdPHcxjEvP2ah7ugGeYo6Qn-sfV2IaTylvZ6vVc0".into(),
            remid: "TUU6V0tPNkVFdURBNE91REI2eGR6YjVvQmFjQ0R4T01BYmFGNURlNDNCcTowMzI5NTQ0MDI.DV2hqKulp6YIU8Wh4XOJxKSAOrxQ905ySGd4NAWD".into(),
            auth_token: "QVQxOjIuMDozLjA6MjQwOmQwQzJLYlRkNHByNmhEdkFvdTdWMVBaZXhYbFNud1MyaVZWOjU0NDAyOnJtbGlo".into(),
        }
    }
}

impl OriginApi {
    pub fn from_cookies(remid: String, sid: String) -> Result<Self> {
        let auth_token = Self::get_auth_token(&remid, &sid)?;
        Ok(Self {
            remid,
            sid,
            auth_token,
        })
    }

    pub fn from_cookies_and_token(remid: String, sid: String, auth_token: String) -> Result<Self> {
        Ok(Self {
            remid,
            sid,
            auth_token,
        })
    }

    /**
    1. make a request to `https://accounts.ea.com/connect/auth?client_id=ORIGIN_JS_SDK&response_type=token&redirect_uri=nucleus:rest&prompt=none&release_type=prod` with the following cookies: `remid` and `sid` (get them from your browser on origin.com)
    **response:**
    ```json
    {
      access_token: 'QVQwOjMuMDozLjA6MjQwOmxUQjhhZEtxRzVucnptQjBwbEZabk1LOEZNS2ljWXVYM0pFOjM4NzcyOnB0cTJs',
      token_type: 'Bearer',
      expires_in: '14399'
    }
    ```
    it lasts 4 hours
    */
    pub fn get_auth_token(remid: &str, sid: &str) -> Result<String> {
        const AUTH_URL: &'static str = "https://accounts.ea.com/connect/auth?client_id=ORIGIN_JS_SDK&response_type=token&redirect_uri=nucleus:rest&prompt=none&release_type=prod";

        let client = reqwest::blocking::Client::new();
        let response: Response = client
            .get(AUTH_URL)
            .header("cookie", format!("remid={remid};sid={sid};"))
            .send()?;

        let auth_data: AuthData = response.json()?;
        Ok(auth_data.access_token)
    }

    pub fn refresh_auth_token(&mut self) -> Result<()> {
        self.auth_token = Self::get_auth_token(&self.remid, &self.sid)?;
        Ok(())
    }

    pub fn authenticated_request<T: std::fmt::Debug + DeserializeOwned>(
        &mut self,
        url: &str,
    ) -> Result<T> {
        let client = reqwest::blocking::Client::new();
        let response: Response = client
            .get(url)
            .header("authtoken", &self.auth_token)
            .send()
            .or_else(|err| {
                if self.refresh_auth_token().is_ok() {
                    client.get(url).header("authtoken", &self.auth_token).send()
                } else {
                    Err(err)
                }
            })?;

        let text = response.text()?;
        let data = quick_xml::de::from_str(&text)?;
        Ok(data)
    }

    /**
    **get id from name:**
    make a request to `https://api3.origin.com/atom/users?eaId=${name}` (name=`Qi-Johnny`, caps doesnt matter) with the following header `authtoken: "QVQwOjMuMDozLjA6MjQwOmxUQjhhZEtxRzVucnptQjBwbEZabk1LOEZNS2ljWXVYM0pFOjM4NzcyOnB0cTJs"`
    **response:**
    ```json
    {
      userId: '1001223352890',
      email: null,
      personaId: '1264838874',
      eaId: 'Qi-Johnny',
      firstName: null,
      lastName: null,
      underageUser: false,
      isDiscoverableEmail: false
    }
    ```
    */
    pub fn get_user_from_name(&mut self, name: &str) -> Result<OriginUser> {
        let url = format!("https://api3.origin.com/atom/users?eaId={name}");
        self.authenticated_request(&url)
    }

    pub fn get_user_id_from_name(&mut self, name: &str) -> Result<String> {
        Ok(self.get_user_from_name(name)?.user_id)
    }

    /**
    **get name from id:**
    make a request to `https://api3.origin.com/atom/users?userIds=${id}` (id=`1001223352890`) with the following header `authtoken: "QVQwOjMuMDozLjA6MjQwOmxUQjhhZEtxRzVucnptQjBwbEZabk1LOEZNS2ljWXVYM0pFOjM4NzcyOnB0cTJs"`
    **response:**
    ```json
    {
      users: [
        {
          userId: '1001223352890',
          email: null,
          personaId: '1264838874',
          eaId: 'Qi-Johnny',
          firstName: null,
          lastName: null,
          underageUser: false,
          isDiscoverableEmail: false
        }
      ]
    }
    ```
    it returns array of users in the order with best matching names (max 20 or 25 users iirc)
    */
    pub fn get_users_from_ids(&mut self, ids: Vec<String>) -> Result<OriginUserList> {
        let url = format!(
            "https://api3.origin.com/atom/users?userIds={}",
            ids.join(",")
        );
        self.authenticated_request(&url)
    }

    pub fn get_user_from_id(&mut self, id: &str) -> Result<OriginUser> {
        let user_list = self.get_users_from_ids(vec![id.to_string()])?;
        let first_user = user_list
            .users
            .into_iter()
            .next()
            .ok_or_else(|| format!("No result for User '{id}'"))?;
        Ok(first_user)
    }

    pub fn get_user_name_from_id(&mut self, id: &str) -> Result<String> {
        Ok(self.get_user_from_id(id)?.ea_id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthData {
    access_token: String,
    token_type: String,
    expires_in: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginUser {
    /// userId
    user_id: String,
    /// email
    email: Option<String>,
    /// personaId
    persona_id: String,
    /// eaId
    #[serde(rename = "EAID")]
    ea_id: String,
    /// firstName
    first_name: Option<String>,
    /// lastName
    last_name: Option<String>,
    /// underageUser
    #[serde(default)]
    underage_user: bool,
    /// isDiscoverableEmail
    #[serde(default)]
    is_discoverable_email: bool,
}

impl OriginUser {
    pub fn print_table(&self) {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);

        table.add_row(Row::new(vec![Cell::new("Field"), Cell::new("Value")]));

        table.add_row(Row::new(vec![
            Cell::new("user_id"),
            Cell::new(&self.user_id),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("email"),
            Cell::new(self.email.as_deref().unwrap_or("None")),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("persona_id"),
            Cell::new(&self.persona_id),
        ]));
        table.add_row(Row::new(vec![Cell::new("ea_id"), Cell::new(&self.ea_id)]));
        table.add_row(Row::new(vec![
            Cell::new("first_name"),
            Cell::new(self.first_name.as_deref().unwrap_or("None")),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("last_name"),
            Cell::new(self.last_name.as_deref().unwrap_or("None")),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("underage_user"),
            Cell::new(&self.underage_user.to_string()),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("is_discoverable_email"),
            Cell::new(&self.is_discoverable_email.to_string().to_string()),
        ]));

        table.printstd();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OriginUserList {
    #[serde(rename = "user")]
    users: Vec<OriginUser>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_name() -> Result<()> {
        let mut api = OriginApi::from_hardcoded();
        let id = "2407904290";
        let name = api.get_user_name_from_id(id)?;
        assert_eq!(name, String::from("w4rm1nd"));
        Ok(())
    }

    #[test]
    fn get_id() -> Result<()> {
        let mut api = OriginApi::from_hardcoded();
        let name = "w4rm1nd";
        let id = api.get_user_id_from_name(name)?;
        assert_eq!(id, String::from("2407904290"));
        Ok(())
    }

    #[test]
    fn xml() -> Result<()> {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><users><user><userId>2407904290</userId><personaId>1711480102</personaId><EAID>w4rm1nd</EAID></user></users>";
        let data: OriginUserList = quick_xml::de::from_str(&xml)?;
        assert!(data.users.len() == 1);
        Ok(())
    }

    #[test]
    fn login() -> Result<()> {
        // assert!(dbg!(OriginApi::from_cookies("TUU6M1lISktrM0o0NDQyamh1WTVQaFh6SDFlU3VYOFYxd0k0OGF6VzFKbTowMzI5MTU5Mzk.bbCxWZDvlydJ878G4typ7XA25OmOZv6x88RArK0Y".to_string(), "UzBJWUY4YnFDb05ka2FFWGN2NksxV1pkN2dQMEN6NVl4UENNQjk2YWY1OUZDVkFlNmIxd0JNNFhLdk82VQ.zycMz3vjH7I7n78mxn1thdUykW7_Gf_aqOSqW5TmWyg".to_string())).is_ok());

        assert!(dbg!(OriginApi::from_cookies("tuu6v0tpnkvfdurbne91rei2egr6yjvvqmfjq0r4t01bymfgnurlndncctowmzi5ntq0mdi.dv2hqkulp6yiu8wh4xojxksaorxq905ysgd4nawd".to_string(), "Uzk0NXRKSmhoclVKWmFGOTJ5aFBIeHd5R25ZaU90MG05R1JSck5zaWNxSXBXMUlCOHNMR1RFVDRGTzMyeQ.NMnvdPHcxjEvP2ah7ugGeYo6Qn-sfV2IaTylvZ6vVc0".to_string())).is_ok());
        Ok(())
    }
}
