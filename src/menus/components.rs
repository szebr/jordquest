use bevy::prelude::Component;

#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct HostPage {}

#[derive(Component)]
pub struct JoinPage {}

#[derive(Component)]
pub struct CreditsPage {}

#[derive(Component)]
pub struct HostButton {}//host button to go to the host page

#[derive(Component)]
pub struct JoinButton {}//join button to go to the join page

#[derive(Component)]
pub struct CreditsButton {}

#[derive(Component)]
pub struct BackToMainMenu {}// back to main menu button

#[derive(Component)]
pub struct HostPortInput {
    pub port: String,
} //host port input field

#[derive(Component)]
pub struct HostPortSaveBut {}//host port save button to save what the user typed in into the network address field

#[derive(Component)]
pub struct NetworkAdresses {
    pub host: String,
    pub port: String,
    pub IPAdress: String,
}