
#[cfg(test)]
use httpmock::{Method::GET, MockServer};
#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};
#[cfg(test)]
use triforce_data_pull::{
    service::DataPull,
    data_pull::serde_models::{
        Event, EventDetails, EventOutter, LeagueForTournaments, Leagues, LiveScheduleOutter,
        LolesportsId, Player, ScheduleOutter, Team, TeamsPlayers, Tournament, Wrapper,
    },
    utils::constants::lolesports,
};
#[cfg(test)]
use color_eyre::Result;
#[cfg(test)]
use serde_json::{json, Value};
#[cfg(test)]
use tokio::fs;

#[cfg(test)]
async fn read_json_file(file_path: &str) -> Result<Value> {
    let content = fs::read_to_string(file_path).await?;
    let json_value: Value = serde_json::from_str(&content)?;
    Ok(json_value)
}

#[cfg(test)]
fn setup() -> DataPull {
    DataPull::default()

}

/// This integration test verifies the correct functionality of `fetch_teams_and_players` function.
///
/// The function sets up a mock HTTP server to provide predefined responses. It then triggers a data fetch operation 
/// and verifies that the correct data has been fetched for a particular team and one of its players.
///
/// Specific checks include verifying team details (like ID, name, image URLs, status), the number of players in the 
/// team, and specific details for one player (like ID, summoner name, role). At the end, the test confirms that the
/// expected HTTP request was sent to the mock server only 1 time.
#[tokio::test]
async fn test_fetch_teams_and_players() -> Result<()> {
    let server = MockServer::start();
    let mock_data = read_json_file("tests/test_data/get_teams.json").await?;

    let mock = server.mock(|when: httpmock::When, then| {
        when.method(GET).path_contains("getTeams");
        then.status(200).json_body(mock_data.clone());
    });
    let mut data_pull = setup();
    data_pull.base_url = server.url("");

    data_pull
        .fetch_teams_and_players()
        .await?;

    assert_eq!(data_pull.teams.len(), 1177);
    assert_eq!(data_pull.players.len(), 6400);
    let team = data_pull.teams.iter().find(|t| t.code == "FNC".to_string());
    assert!(team.is_some());

    let fnatic = team.unwrap();
    assert_eq!(fnatic.id.0, 98767991866488695);
    assert_eq!(fnatic.slug, "fnatic");
    assert_eq!(fnatic.name, "Fnatic");
    assert_eq!(fnatic.code, "FNC");
    assert_eq!(fnatic.image, "http://static.lolesports.com/teams/1631819669150_fnc-2021-worlds.png");
    assert_eq!(fnatic.alternative_image, Some("http://static.lolesports.com/teams/1592591295310_FnaticFNC-03-FullonLight.png".to_string()));
    assert_eq!(fnatic.background_image, Some("http://static.lolesports.com/teams/1632941274242_FNC.png".to_string()));
    assert_eq!(fnatic.status, "active");
    assert!(fnatic.home_league.is_some());
    assert_eq!(fnatic.players.len(), 8);

    let home_league = fnatic.home_league.clone().unwrap();
    assert_eq!(home_league.name, "LEC");
    assert_eq!(home_league.region, "EMEA");
    let player = fnatic.players.iter().find(|p|p.id.0 == 100356590519370319);
    assert!(player.is_some());
    let humanoid = player.unwrap();
    assert_eq!(humanoid.id.0, 100356590519370319);
    assert_eq!(humanoid.summoner_name, "Humanoid");
    assert_eq!(humanoid.first_name, " Marek");
    assert_eq!(humanoid.last_name, "Brázda");
    assert_eq!(humanoid.image, Some("http://static.lolesports.com/players/1674150706185_humanoid.png".to_string()));
    assert_eq!(humanoid.role, "mid");

    mock.assert();

    Ok(())
}