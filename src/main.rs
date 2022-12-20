#![allow(dead_code)]
use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::env;
use std::fs;

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

    /// Via. Must be "longitude,latitude" format.
    #[clap(short = 'v', long = "via", default_value = None)]
    via: Option<String>,

    /// Departure datetime. "yyyyMMdd_HHmmss.
    #[clap(short = 'd', long = "date", default_value = None)]
    date: Option<String>,

    /// Output to file.
    #[clap(short = 'o', long = "output", default_value = None)]
    file: Option<String>,
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
    /// Rapid API key
    api_key: String,

    /// Rapid API Host
    api_host: String,
}

impl RequestHeader {
    fn new() -> Self {
        let api_key = env::var("RAPID_API_KEY").expect("RAPID_API_KEY is not set");
        Self {
            api_key,
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

    fn new_with_result_id(result_id: String) -> Self {
        Self {
            routeresultid: Some(result_id),
            ..Default::default()
        }
    }

    fn via(&mut self, via: String) -> &mut Self {
        self.via = Some(via);
        self
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

    fn date(&mut self, date: String) -> &mut Self {
        // TODO: check date format yyyyMMdd_HHmmss
        self.date = Some(date);
        self
    }

    fn to_params(self) -> Vec<(String, String)> {
        let mut p = vec![];
        if let Some(result_id) = self.routeresultid {
            p.push(("routeresultid".to_string(), result_id));
        } else {
            p.push(("start".to_string(), self.start));
            p.push(("destination".to_string(), self.destination));
            if let Some(via) = self.via {
                p.push((
                    "via".to_string(),
                    via
                ));
            }
            if let Some(cartype) = self.cartype {
                p.push((
                    "cartype".to_string(),
                    serde_json::to_string(&cartype).unwrap(),
                ));
            }
            if let Some(date) = self.date {
                p.push((
                    "date".to_string(),
                    date,
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

    /// for toll price
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

    /// road reguration accordingly
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

    /// Get route result(have to set either start,destination or routeresultid)
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
    #[serde(rename = "routeId")]
    route_id: Option<String>,
    status: Option<String>,
    #[serde(rename = "routeResultId")]
    route_result_id: Option<String>,
    summary: Option<RouteSummary>,
    guide: Option<Vec<Guide>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Guide {
    #[serde(rename = "type")]
    type_: Option<GuideType>,
    #[serde(rename = "guidePoints")]
    guide_points: Option<Vec<Point>>,
    #[serde(rename = "guideInfo")]
    guide_info: Option<GuideInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuideInfo {
    #[serde(rename = "guideDirection")]
    guide_direction: Option<GuideDirection>,

    #[serde(rename = "roadType")]
    road_type: Option<u16>,

    distance: Option<f64>,

    #[serde(rename = "travelTime")]
    travel_time: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "guideDetail")]
    guide_detail: Option<GuideDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "guideHighway")]
    guide_highway: Option<GuideHighway>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "guideCrossing")]
    guide_crossing: Option<GuideCrossing>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "guideRoad")]
    guide_road: Option<GuideRoad>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "guideToll")]
    guide_toll: Option<GuideToll>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "guideTollEtc")]
    guide_toll_etc: Option<GuideTollEtc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shapeIndexFirst")]
    shape_index_first: Option<ShapeIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shapeIndexLast")]
    shape_index_last: Option<ShapeIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shape: Option<Vec<ShapeType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shapeInfo")]
    shape_info: Option<ShapeInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shapePoints")]
    shape_points: Option<Vec<ShapePoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<Vec<u32>>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum GuideDirection {
    Unknown = 0,
    Along = 1,
    Straight = 2,
    Right30 = 3,
    Right45 = 4,
    Right = 5,
    Right135 = 6,
    Right150 = 7,
    Uturn = 8,
    Left150 = 9,
    Left135 = 10,
    Left = 11,
    Left45 = 12,
    Left30 = 13,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum RoadType {
    Ineligible = 0,
    NormalCountry = 1,
    MainLocal = 2,
    MainLocalCity = 3,
    NormalLocal = 4,
    NormalLocalCity = 5,
    Other1 = 6,
    Other2 = 7,
    NarrowLocalRoad1 = 8,
    NarrowLocalRoad2 = 9,
    NarrowLocalRoad3 = 10,
    //12 ~ 99 reserved
    //100 reserved
    Highway = 101,
    CityHighway = 102,
    NormalCountryToll = 103,
    MainLocalToll = 104,
    MainLocalCityToll = 105,
    NormalLocalToll = 106,
    NormalLocalCityToll = 107,
    OtherToll = 108,
    //109 ~ 199 reserved
    //Ferry = 200 - 299,
    //OtherNormal = 300 ~ 399
}

#[derive(Serialize, Deserialize, Debug)]
struct ShapePoint {
    lon: Option<f32>,
    lat: Option<f32>,
    el: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ShapeInfo {
    #[serde(rename = "roadType")]
    road_type: Option<u8>,
    #[serde(rename = "dataId")]
    data_id: Option<u8>,
    // bitwise operation is necessary
    //属性
    //0 オートウォーク
    //1 階段
    //2 スロープ
    //3 エスカレータ
    //4 屋根付き
    //5 トンネル
    //6 広場
    //7 エレベータ
    //11-8 (リザーブ)
    //15-12 通行禁止種別
    //19-16 一方通行種別
    info: Option<u32>,
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
    #[serde(rename = "shapeIndex")]
    shape_index: Option<u16>,
    #[serde(rename = "shapePointsIndex")]
    shape_points_index: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuideTollEtc {
    #[serde(rename = "tollGateCode")]
    toll_gate_code: Option<TollGateCode>,
    toll: Option<i64>,
    name: Option<String>,
    #[serde(rename = "etcCode")]
    etc_code: Option<EtcCode>,
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
    #[serde(rename = "tollGateCode")]
    toll_gate_code: Option<TollGateCode>,
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
    // bitwize operation is necessary
    // bit
    // 施設
    // 7-0 (リザーブ)
    // 8 トイレ
    // 9 身障者用トイレ
    // 10 レストラン
    // 11 軽食
    // 12 売店
    // 13 休憩所
    // 14 仮眠休憩所
    // 15 対人案内所
    // 16 インフォメーション
    // 17 シャワー施設
    // 18 コインランドリー
    // 19 公衆浴場
    // 20 FAX
    // 21 郵便ポスト
    // 22 キャッシュディスペンサーサービス
    // 23 ハイウェイオアシス
    // 24 コイン洗車場
    // 25 ガソリンスタンド
    info: Option<u32>,
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
    #[serde(rename = "totalDistance")]
    total_distance: Option<f64>,

    #[serde(rename = "totalTravelTime")]
    total_travel_time: Option<f64>,

    #[serde(rename = "totalToll")]
    total_toll: Option<Toll>,

    #[serde(rename = "totalTollEtc")]
    total_toll_etc: Option<Toll>,

    #[serde(rename = "departureTime")]
    departure_time: Option<DateTime>,

    #[serde(rename = "sectionTime")]
    section_time: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Toll {
    toll: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DateTime {
    /// yyyyMMdd
    date: Option<String>,

    /// HHmmss
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
    if let Some(date) = route_args.date {
        params.date(date);
    }
    if let Some(via) = route_args.via {
        params.via(via);
    }

    let url = reqwest::Url::parse_with_params(BASE_URL, params.to_params())?;
    let client = reqwest::Client::new();
    let req = client
        .get(url)
        .header("X-RapidAPI-Key", header.api_key)
        .header("X-RapidAPI-Host", header.api_host);
    let res = req.send().await?;
    anyhow::ensure!(res.status() == StatusCode::OK, "{:?}", res);

    let output = res.text().await?;
    let obj: RouteResult = serde_json::from_str(&output).unwrap();
    let json_str = serde_json::to_string(&obj).unwrap();

    if let Some(file) = route_args.file {
        fs::write(file, &json_str)?;
    } else {
        println!("{}", json_str);
    }

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
        assert_eq!(
            params.to_params(),
            vec![
                ("start".to_string(), "".to_string()),
                ("destination".to_string(), "".to_string()),
                ("tollroad".to_string(), "1".to_string())
            ]
        );

        let params = CalcRouteRequestParam {
            tollroad: Some(OnOff::Off),
            ..Default::default()
        };
        assert_eq!(
            params.to_params(),
            vec![
                ("start".to_string(), "".to_string()),
                ("destination".to_string(), "".to_string()),
                ("tollroad".to_string(), "0".to_string())
            ]
        );
    }
    #[test]
    fn date_should_be_date() {
        let params = CalcRouteRequestParam {
            date: Some("20221204_100000".to_string()),
            ..Default::default()
        };
        assert_eq!(
            params.to_params(),
            vec![
                ("start".to_string(), "".to_string()),
                ("destination".to_string(), "".to_string()),
                ("date".to_string(), "20221204_100000".to_string())
            ]
        );
    }

}
