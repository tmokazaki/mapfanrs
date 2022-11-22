use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::env;

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// Execute Route
    Route(RouteArgs),
}

#[derive(Debug, Parser)]
struct RouteArgs {
    /// Origin. Must be "longitude,latitude" format.
    #[clap(short = 'f', long = "from")]
    from: String,

    /// Destination. Must be "longitude,latitude" format.
    #[clap(short = 't', long = "to")]
    to: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let main_args = Arguments::parse();
    //println!("{:?}", args);
    match main_args.command {
        SubCommand::Route(route_args) => handle_route(route_args).await,
    }
}

struct RequestHeader {
    api_key: String,
    api_host: String,
}

impl RequestHeader {
    fn new() -> Self {
        let api_key = env::var("RAPID_API_KEY").expect("RAPID_API_KEY is not set");
        Self {
            api_key: api_key,
            api_host: "mapfanapi-route.p.rapidapi.com".to_string(),
        }
    }
}

struct Position {
    longitude: f32,
    latitude: f32,
    // type
}

impl CalcRouteRequestParam {
    fn new(start: Position, destination: Position) -> Self {
        Self {
            start: format!("{},{}", start.longitude, start.latitude),
            destination: format!("{},{}", destination.longitude, destination.latitude),
            ..Default::default()
        }
    }

    fn newWithResultId(resultId: String) -> Self {
        Self {
            routeresultid: Some(resultId),
            ..Default::default()
        }
    }

    fn car_type(&mut self, cartype: CarType) -> &mut Self {
        self.cartype = Some(cartype);
        self
    }

    fn vehicle_type(&mut self, vehicletype: VehicleType) -> &mut Self {
        self.vehicletype = Some(vehicletype);
        self
    }

    fn result_type(&mut self, resulttype: OnOff) -> &mut Self {
        self.resulttype = Some(resulttype);
        self
    }

    fn to_params(self) -> Vec<(String, String)> {
        let mut p = vec![];
        if let Some(result_id) = self.routeresultid {
            p.push(("routeresultid".to_string(), result_id));
        } else {
            p.push(("start".to_string(), self.start));
            p.push(("destination".to_string(), self.destination));
            if let Some(cartype) = self.cartype {
                p.push((
                    "cartype".to_string(),
                    serde_json::to_string(&cartype).unwrap(),
                ));
            }
            if let Some(resulttype) = self.resulttype {
                p.push((
                    "resulttype".to_string(),
                    serde_json::to_string(&resulttype).unwrap(),
                ));
            }
            if let Some(vehicletype) = self.vehicletype {
                p.push((
                    "vehicletype".to_string(),
                    serde_json::to_string(&vehicletype).unwrap(),
                ));
            }
            if let Some(tollroad) = self.tollroad {
                p.push((
                    "tollroad".to_string(),
                    serde_json::to_string(&tollroad).unwrap(),
                ));
            }
        }
        p
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct CalcRouteRequestParam {
    start: String,
    destination: String,
    /// starting angle 0 ~ 359
    #[serde(skip_serializing_if = "Option::is_none")]
    startangle: Option<i16>,
    /// 'longitude,latitude,type,priority|longitude,latitude,type,priority|...'
    #[serde(skip_serializing_if = "Option::is_none")]
    via: Option<String>,
    /// departure date "yyyyMMdd_HHmmss"
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tollway: Option<Tollway>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ferry: Option<Ferry>,
    /// Smart IC. use: 1, not_use: 0, default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    smartic: Option<OnOff>,
    /// ETC. use: 1, not_use: 0, default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    etc: Option<OnOff>,
    /// normal + etc discount: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    tolltarget: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cartype: Option<CarType>,
    /// speed on normal way
    #[serde(skip_serializing_if = "Option::is_none")]
    normalspeed: Option<f32>,
    /// speed on highway
    #[serde(skip_serializing_if = "Option::is_none")]
    highwayspeed: Option<f32>,
    /// speed on tall way
    #[serde(skip_serializing_if = "Option::is_none")]
    tollwayspeed: Option<f32>,
    /// speed on ferry
    #[serde(skip_serializing_if = "Option::is_none")]
    ferryspeed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vehicletype: Option<VehicleType>,
    /// height of the vehicle(cm)
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i32>,
    /// loadage(kg)
    #[serde(skip_serializing_if = "Option::is_none")]
    loadage: Option<i32>,
    /// weight of the vehicle(kg)
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<i32>,
    /// width of the vehicle(cm)
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i32>,
    /// cargo with danger: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    danger: Option<u8>,
    /// restrict daytime: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    daytime: Option<u8>,
    /// enable restrict general road: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    generalroad: Option<u8>,
    /// enable restrict toll road: 1, default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    tollroad: Option<OnOff>,
    /// enable oneway restriction: 1, default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    regulations: Option<OnOff>,
    /// travel route: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    travel: Option<OnOff>,
    //passablearea: Option<String>,
    //impassablearea: Option<String>,
    /// avoid Uturn
    //uturnavoid: Option<u8>,
    /// choose Uturn
    //uturn: Option<u8>,
    /// ID of this request
    //routeid: Option<String>,
    /// Get additional ID for Route. default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    resulttype: Option<OnOff>,
    /// Get route result
    #[serde(skip_serializing_if = "Option::is_none")]
    routeresultid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fmt: Option<OutputFormat>,
}

#[derive(Serialize, Deserialize, Debug)]
enum OutputFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "xml")]
    Xml,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u16)]
enum Priority {
    Normal = 0,
    DistanceFirst = 1,
    StraightFirst = 2,
    SimpleWalker = 3,
    RoadWidthFirst = 4,
    NormalWalker = 100,
    WalkerDistanceFirst = 101,
    WalkerRoofFirst = 102,
    WalkerLessSteps = 103,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum Tollway {
    Normal = 0,
    Priority = 1,
    Avoid = 2,
    Never = 3,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum Ferry {
    Normal = 0,
    Priority = 1,
    Avoid = 2,
    Never = 3,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum CarType {
    /// 軽自動車
    Small = 0,
    /// 普通車
    Normal = 1,
    /// 中型車
    Middle = 2,
    /// 大型車
    Big = 3,
    /// 特大車
    SuperBig = 4,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum VehicleType {
    None = 0,
    /// 大型乗用自動車
    Big = 1,
    /// 大型貨物自動車
    BigCargo = 2,
    /// 大型特殊自動車
    BigSpecial = 11,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum OnOff {
    Off = 0,
    On = 1,
}

#[derive(Serialize, Deserialize, Debug)]
struct RouteResult {
    routeId: Option<String>,
    status: Option<String>,
    routeResultId: Option<String>,
    summary: Option<RouteSummary>,
    guide: Option<Vec<Guide>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Guide {
    #[serde(rename = "type")]
    type_: Option<GuideType>,
    guidePoints: Option<Vec<Point>>,
    guideInfo: Option<GuideInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuideInfo {
    guideDirection: Option<u8>,
    roadType: Option<u8>,
    distance: Option<f64>,
    travelTime: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guideDetail: Option<GuideDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guideHighway: Option<GuideHighway>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guideCrossing: Option<GuideCrossing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guideRoad: Option<GuideRoad>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guideToll: Option<GuideToll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guideTollEtc: Option<GuideTollEtc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shapeIndexFirst: Option<ShapeIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shapeIndexLast: Option<ShapeIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shape: Option<Vec<ShapeType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shapeInfo: Option<ShapeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shapePoints: Option<Vec<ShapePoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ShapePoint {
    lon: Option<f32>,
    lat: Option<f32>,
    el: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ShapeInfo {
    roadType: Option<u8>,
    dataId: Option<u8>,
    info: Option<u8>,
    distance: Option<f64>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum ShapeType {
    Road = 4,
    Start = 5,
    End = 6,
}

#[derive(Serialize, Deserialize, Debug)]
struct ShapeIndex {
    shapeIndex: Option<u16>,
    shapePointsIndex: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuideTollEtc {
    tollGateCode: Option<TollGateCode>,
    toll: Option<i64>,
    name: Option<String>,
    etcCode: Option<EtcCode>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum EtcCode {
    Unsupported = 0,
    Gate = 1,
    Antena = 2,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuideToll {
    tollGateCode: Option<TollGateCode>,
    toll: Option<i64>,
    name: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum TollGateCode {
    Issue = 1,
    Settle = 2,
    SimpleGate = 3,
    SimpleGateAndIssue = 4,
    SimpleGateAndSettle = 5,
    UturnCheck = 6,
    InvalidIssue = 7,
    SettleAndIssue = 8,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuideRoad {
    number: Option<u16>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuideCrossing {
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuideHighway {
    facilities: Option<Vec<Facility>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Facility {
    #[serde(rename = "type")]
    type_: Option<FacilityType>,
    name: Option<String>,
    info: Option<u8>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum FacilityType {
    Sa = 1,
    Pa = 2,
    Junction = 3,
    Rump = 4,
    Ic = 5,
    SmartIc = 7,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuideDetail {
    code: Option<GuideDetailCode>,
    name: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum GuideDetailCode {
    HighwayEntrance = 32,
    HighwayExit = 33,
    HighwayService = 34,
    FerryTerminal = 48,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    lon: Option<f32>,
    lat: Option<f32>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum GuideType {
    Point = 0,
    Start = 1,
    Goal = 2,
    Waypoint = 3,
}

#[derive(Serialize, Deserialize, Debug)]
struct RouteSummary {
    totalDistance: Option<f64>,
    totalTravelTime: Option<f64>,
    totalToll: Option<Toll>,
    totalTollEtc: Option<Toll>,
    departureTime: Option<DateTime>,
    sectionTime: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Toll {
    toll: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DateTime {
    date: Option<String>,
    time: Option<String>,
}

async fn handle_route(route_args: RouteArgs) -> Result<()> {
    const BASE_URL: &str = "https://mapfanapi-route.p.rapidapi.com/calcroute";
    let header = RequestHeader::new();
    let origin: Vec<f32> = route_args
        .from
        .split(",")
        .map(|v| v.parse::<f32>().expect("input must be a float number"))
        .collect();
    let dest: Vec<f32> = route_args
        .to
        .split(",")
        .map(|v| v.parse::<f32>().expect("input must be a float number"))
        .collect();
    anyhow::ensure!(
        origin.len() == 2 && dest.len() == 2,
        "invalid from/to parameter, it must be 'lon,lat' format. from: {}, to: {}",
        route_args.from,
        route_args.to
    );

    let start = Position {
        longitude: origin[0],
        latitude: origin[1],
    };
    let destination = Position {
        longitude: dest[0],
        latitude: dest[1],
    };
    let mut params = CalcRouteRequestParam::new(start, destination);
    params.vehicle_type(VehicleType::BigCargo);

    let url = reqwest::Url::parse_with_params(BASE_URL, params.to_params())?;
    let client = reqwest::Client::new();
    let req = client
        .get(url)
        .header("X-RapidAPI-Key", header.api_key)
        .header("X-RapidAPI-Host", header.api_host);
    let res = req.send().await?;
    anyhow::ensure!(res.status() == StatusCode::OK, "{:?}", res);

    let obj: RouteResult = serde_json::from_str(&res.text().await?).unwrap();
    let json_str = serde_json::to_string(&obj).unwrap();
    println!("{}", json_str);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn onoff_should_be_number() {
        let params = CalcRouteRequestParam {
            tollroad: Some(OnOff::On),
            ..Default::default()
        };
        assert_eq!(params.to_params(), vec![
            ("start".to_string(), "".to_string()), ("destination".to_string(), "".to_string()),
            ("tollroad".to_string(), "1".to_string())
        ]);

        let params = CalcRouteRequestParam {
            tollroad: Some(OnOff::Off),
            ..Default::default()
        };
        assert_eq!(params.to_params(), vec![
            ("start".to_string(), "".to_string()), ("destination".to_string(), "".to_string()),
            ("tollroad".to_string(), "0".to_string())
        ]);
    }
}
