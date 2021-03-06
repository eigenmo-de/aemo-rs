/// # Summary
/// 
/// ## P5MIN_INTERSENSITIVITIES
///  _Price Sensitivies for 5MinPD solution. New solution every 5 minutes. Current Scenarios defined in P5MIN_SCENARIODEMANDTRK/P5MIN_SCENARIODEMAND_
/// 
/// * Data Set Name: P5min
/// * File Name: Intersensitivities
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INTERCONNECTORID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minIntersensitivities1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Interconnector identifier
    pub interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    pub intervention: rust_decimal::Decimal,
    /// Flag to indicate if this period has an active intervention constraint: 0= No, 1= Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow1 = MW flow for given Interconnector for Scenario 1
    pub mwflow1: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow2 = MW flow for given Interconnector for Scenario 2
    pub mwflow2: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow3 = MW flow for given Interconnector for Scenario 3
    pub mwflow3: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow4 = MW flow for given Interconnector for Scenario 4
    pub mwflow4: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow5 = MW flow for given Interconnector for Scenario 5
    pub mwflow5: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow6 = MW flow for given Interconnector for Scenario 6
    pub mwflow6: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow7 = MW flow for given Interconnector for Scenario 7
    pub mwflow7: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow8 = MW flow for given Interconnector for Scenario 8
    pub mwflow8: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow9 = MW flow for given Interconnector for Scenario 9
    pub mwflow9: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow10 = MW flow for given Interconnector for Scenario 10
    pub mwflow10: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow11 = MW flow for given Interconnector for Scenario 11
    pub mwflow11: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow12 = MW flow for given Interconnector for Scenario 12
    pub mwflow12: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow13 = MW flow for given Interconnector for Scenario 13
    pub mwflow13: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow14 = MW flow for given Interconnector for Scenario 14
    pub mwflow14: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow15 = MW flow for given Interconnector for Scenario 15
    pub mwflow15: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow16 = MW flow for given Interconnector for Scenario 16
    pub mwflow16: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow17 = MW flow for given Interconnector for Scenario 17
    pub mwflow17: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow18 = MW flow for given Interconnector for Scenario 18
    pub mwflow18: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow19 = MW flow for given Interconnector for Scenario 19
    pub mwflow19: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow20 = MW flow for given Interconnector for Scenario 20
    pub mwflow20: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow21 = MW flow for given Interconnector for Scenario 21
    pub mwflow21: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow22 = MW flow for given Interconnector for Scenario 22
    pub mwflow22: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow23 = MW flow for given Interconnector for Scenario 23
    pub mwflow23: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow24 = MW flow for given Interconnector for Scenario 24
    pub mwflow24: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow25 = MW flow for given Interconnector for Scenario 25
    pub mwflow25: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow26 = MW flow for given Interconnector for Scenario 26
    pub mwflow26: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow27 = MW flow for given Interconnector for Scenario 27
    pub mwflow27: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow28 = MW flow for given Interconnector for Scenario 28
    pub mwflow28: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow29 = MW flow for given Interconnector for Scenario 29
    pub mwflow29: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow30 = MW flow for given Interconnector for Scenario 30
    pub mwflow30: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow31 = MW flow for given Interconnector for Scenario 31
    pub mwflow31: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow32 = MW flow for given Interconnector for Scenario 32
    pub mwflow32: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow33 = MW flow for given Interconnector for Scenario 33
    pub mwflow33: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow34 = MW flow for given Interconnector for Scenario 34
    pub mwflow34: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow35 = MW flow for given Interconnector for Scenario 35
    pub mwflow35: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow36 = MW flow for given Interconnector for Scenario 36
    pub mwflow36: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow37 = MW flow for given Interconnector for Scenario 37
    pub mwflow37: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow38 = MW flow for given Interconnector for Scenario 38
    pub mwflow38: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow39 = MW flow for given Interconnector for Scenario 39
    pub mwflow39: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow40 = MW flow for given Interconnector for Scenario 40
    pub mwflow40: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow41 = MW flow for given Interconnector for Scenario 41
    pub mwflow41: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow42 = MW flow for given Interconnector for Scenario 42
    pub mwflow42: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow43 = MW flow for given Interconnector for Scenario 43
    pub mwflow43: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for P5minIntersensitivities1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("INTERSENSITIVITIES".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## P5MIN_UNITSOLUTION
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_UNITSOLUTION shows the Unit results from the capacity evaluations for each period of the study.<br>_
/// 
/// * Data Set Name: P5min
/// * File Name: Unitsolution
/// * Data Version: 3
/// 
/// # Description
///  P5MIN_UNITSOLUTION data is confidential, so shows own details for participant. Source P5MIN_UNITSOLUTION updates every 5 minutes for all units, even zero targets. Volume Rows per day: 57600 Based on 200 units per Interval Note A bitwise flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The SPD Formulation document details the logic determining whether a unit is "trapped" or "stranded". The flag is defined as follows: Flagged Condition Bit Description Field value FCAS profile active 0 The bid profile for this service has been activated such that the unit is available to be cleared to provide this ancillary service type. 1 or 3 Trapped 1 The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market. 3 Stranded 2 The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service. 4
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minUnitsolution3 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Connection point identifier for DUID
    pub connectionpointid: Option<String>,
    /// Generator or Load
    pub tradetype: Option<rust_decimal::Decimal>,
    /// AGC Status from EMS: 1 = on, 0 = off
    pub agcstatus: Option<rust_decimal::Decimal>,
    /// Initial MW at start of period. For periods subsequent to the first period of a P5MIN run, this value represents the cleared target for the previous period of that P5MIN run.
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Target MW for end of period
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Ramp down rate (lessor of bid or telemetered rate).
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate (lessor of bid or telemetered rate).
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Lower 5 min reserve target
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 sec reserve target
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 sec reserve target
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 min reserve target
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 sec reserve target
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 sec reserve target
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Energy Availability (MW)
    pub availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag 
    pub raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag  
    pub raise60secflags: Option<rust_decimal::Decimal>,
    /// Raise 5min status flag  
    pub raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise Reg status flag  
    pub raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag 
    pub lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag  
    pub lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag  
    pub lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag  
    pub lowerregflags: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Boolean representation flagging if the Target is Capped
    pub semidispatchcap: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run(INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minUnitsolution3 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("UNITSOLUTION".into()),
                        version: 3,
                    }
                    
    }
}
/// # Summary
/// 
/// ## P5MIN_SCENARIODEMAND
///  _The P5Min scenario MW offsets_
/// 
/// * Data Set Name: P5min
/// * File Name: Scenariodemand
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * REGIONID
/// * SCENARIO
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minScenariodemand1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// The scenario identifier
    pub scenario: rust_decimal::Decimal,
    /// The region to which to apply the deltaMW for this SCENARIO
    pub regionid: String,
    /// The MW offset to apply to region total demand for this SCENARIO
    pub deltamw: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minScenariodemand1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("SCENARIODEMAND".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## P5MIN_LOCAL_PRICE
///  _Sets out local pricing offsets associated with each DUID connection point for each dispatch period_
/// 
/// * Data Set Name: P5min
/// * File Name: Local Price
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minLocalPrice1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    pub local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minLocalPrice1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("LOCAL_PRICE".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## P5MIN_REGIONSOLUTION
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_REGIONSOLUTION shows the results of the regional capacity, maximum surplus reserve and maximum spare capacity evaluations for each period of the study.<br>_
/// 
/// * Data Set Name: P5min
/// * File Name: Regionsolution
/// * Data Version: 6
/// 
/// # Description
///  P5MIN_REGIONSOLUTION is public data, so is available to all participants. Source P5MIN_REGIONSOLUTION updates every 5 minutes. Volume Rows per day: 1440
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minRegionsolution6 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region Identifier
    pub regionid: String,
    /// Region Reference Price (Energy)
    pub rrp: Option<rust_decimal::Decimal>,
    /// Region Override Price (Energy)
    pub rop: Option<rust_decimal::Decimal>,
    /// Total Energy Imbalance (MW)
    pub excessgeneration: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise6Sec)
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise6Sec)
    pub raise6secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise60Sec)
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise60Sec)
    pub raise60secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise5Min)
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise5Min)
    pub raise5minrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (RaiseReg)
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (RaiseReg)
    pub raiseregrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower6Sec)
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower6Sec)
    pub lower6secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower60Sec)
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower60Sec)
    pub lower60secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower5Min)
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower5Min)
    pub lower5minrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (LowerReg)
    pub lowerregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (LowerReg)
    pub lowerregrop: Option<rust_decimal::Decimal>,
    /// Regional Demand - NB NOT net of Interconnector flows or Loads
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Regional Available generation
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Regional Available Load
    pub availableload: Option<rust_decimal::Decimal>,
    /// Predicted change in regional demand for this interval
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Regional Generation Dispatched
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Regional Load Dispatched
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector Flows
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW dispatch
    pub lower5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW imported
    pub lower5minimport: Option<rust_decimal::Decimal>,
    /// Lower 5 min local dispatch
    pub lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min local requirement
    pub lower5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min total requirement
    pub lower5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW dispatch
    pub lower60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW imported
    pub lower60secimport: Option<rust_decimal::Decimal>,
    /// Lower 60 sec local dispatch
    pub lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec local requirement
    pub lower60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec total requirement
    pub lower60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW dispatch
    pub lower6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW imported
    pub lower6secimport: Option<rust_decimal::Decimal>,
    /// Lower 6 sec local dispatch
    pub lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec local requirement
    pub lower6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec total requirement
    pub lower6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Total Raise 5 min MW dispatch
    pub raise5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW imported
    pub raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    pub raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min local requirement
    pub raise5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min total requirement
    pub raise5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW dispatch
    pub raise60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW imported
    pub raise60secimport: Option<rust_decimal::Decimal>,
    /// Raise 50 sec local dispatch
    pub raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec local requirement
    pub raise60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec total requirement
    pub raise60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW dispatch
    pub raise6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW imported
    pub raise6secimport: Option<rust_decimal::Decimal>,
    /// Raise 6 sec local dispatch
    pub raise6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec local requirement
    pub raise6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec total requirement
    pub raise6secreq: Option<rust_decimal::Decimal>,
    /// Aggregate dispatch error applied
    pub aggregatedispatcherror: Option<rust_decimal::Decimal>,
    /// Sum of initial generation and import for region
    pub initialsupply: Option<rust_decimal::Decimal>,
    /// Sum of cleared generation and import for region
    pub clearedsupply: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation MW imported
    pub lowerregimport: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Total Lower Regulation dispatch
    pub lowerregdispatch: Option<rust_decimal::Decimal>,
    /// Lower Regulation local dispatch
    pub lowerreglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation local requirement
    pub lowerreglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation total requirement
    pub lowerregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation MW imported
    pub raiseregimport: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Total Raise Regulation dispatch
    pub raiseregdispatch: Option<rust_decimal::Decimal>,
    /// Raise Regulation local dispatch
    pub raisereglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation local requirement
    pub raisereglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation total requirement
    pub raiseregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 5 min local requirement
    pub raise5minlocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise Reg local requirement
    pub raisereglocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 60 sec local requirement
    pub raise60seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 6 sec local requirement
    pub raise6seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 5 min local requirement
    pub lower5minlocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower Reg local requirement
    pub lowerreglocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 60 sec local requirement
    pub lower60seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 6 sec local requirement
    pub lower6seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 5 min requirement
    pub raise5minviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise Reg requirement
    pub raiseregviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 60 seconds requirement
    pub raise60secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 6 seconds requirement
    pub raise6secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 5 min requirement
    pub lower5minviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower Reg requirement
    pub lowerregviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 60 seconds requirement
    pub lower60secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 6 seconds requirement
    pub lower6secviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW).
    pub totalintermittentgeneration: Option<rust_decimal::Decimal>,
    /// Sum of Cleared Scheduled generation, imported generation (at the region boundary) and allowances made for non-scheduled generation (MW).
    pub demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW).
    pub uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW
    pub semischedule_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced
    pub semischedule_compliancemw: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is solar
    pub ss_solar_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is wind
    pub ss_wind_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where the primary fuel source is solar
    pub ss_solar_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where the primary fuel source is wind
    pub ss_wind_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced and the primary fuel source is solar
    pub ss_solar_compliancemw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced and the primary fuel source is wind
    pub ss_wind_compliancemw: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minRegionsolution6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("REGIONSOLUTION".into()),
                        version: 6,
                    }
                    
    }
}
/// # Summary
/// 
/// ## P5MIN_INTERCONNECTORSOLN
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_INTERCONNECTORSOLN sets out the results of the capacity evaluation for Interconnectors, including the calculated limits for the interval.<br>_
/// 
/// * Data Set Name: P5min
/// * File Name: Interconnectorsoln
/// * Data Version: 4
/// 
/// # Description
///  P5MIN_INTERCONNECTORSOLN is public data, so is available to all participants. Source P5MIN_INTERCONNECTORSOLN updates every 5 minutes. Volume Rows per day: 1440 Based on 200 interconnector/binding constraints per interval
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INTERCONNECTORID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minInterconnectorsoln4 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Interconnector identifier
    pub interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// SCADA MW Flow measured at Run start. For periods subsequent to the first period of a P5MIN run, this value represents the cleared target for the previous period of that P5MIN run.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Cleared Interconnector loading level (MW)
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Interconnector Losses at cleared flow
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// Marginal cost of Interconnector standing data limits (if binding)
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Violation of Interconnector standing data limits
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Flag indicating MNSP registration
    pub mnsp: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit. Note unlike the input interconnector import limit this is a directional quantity and should be defined with respect to the interconnector flow.
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor at the cleared flow
    pub marginalloss: Option<rust_decimal::Decimal>,
    /// Generic Constraint setting the export limit
    pub exportgenconid: Option<String>,
    /// Generic Constraint setting the import limit
    pub importgenconid: Option<String>,
    /// Calculated export limit applying to energy + Frequency Controlled Ancillary Services.
    pub fcasexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy + Frequency Controlled Ancillary Services.
    pub fcasimportlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Export (Factor &gt;= 0)
    pub local_price_adjustment_export: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Export: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_export: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Import (Factor &gt;= 0)
    pub local_price_adjustment_import: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Import: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_import: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    pub intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minInterconnectorsoln4 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("INTERCONNECTORSOLN".into()),
                        version: 4,
                    }
                    
    }
}
/// # Summary
/// 
/// ## P5MIN_BLOCKEDCONSTRAINT
///  _P5MIN Blocked Constraints lists any constraints that were blocked in a P5MIN run. If no constraints are blocked, there will be no rows for that 5 minute predispatch run._
/// 
/// * Data Set Name: P5min
/// * File Name: Blocked Constraints
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * CONSTRAINTID
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minBlockedConstraints1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
}
impl crate::GetTable for P5minBlockedConstraints1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("BLOCKED_CONSTRAINTS".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## P5MIN_SCENARIODEMANDTRK
///  _Tracks the 5Min scenario offset updates across time_
/// 
/// * Data Set Name: P5min
/// * File Name: Scenariodemandtrk
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minScenariodemandtrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for P5minScenariodemandtrk1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("SCENARIODEMANDTRK".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## P5MIN_CASESOLUTION
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_CASESOLUTION shows one record containing results pertaining to the entire solution.<br>_
/// 
/// * Data Set Name: P5min
/// * File Name: Casesolution
/// * Data Version: 2
/// 
/// # Description
///  P5MIN_CASESOLUTION data is public, so is available to all participants. Source P5MIN_CASESOLUTION updates every 5 minutes. Volume Rows per day: 288
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minCasesolution2 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Date and Time of first interval in study
    pub startinterval_datetime: Option<String>,
    /// The Objective function from the LP
    pub totalobjective: Option<rust_decimal::Decimal>,
    /// Flag to indicate non-physical losses occurred in this study
    pub nonphysicallosses: Option<rust_decimal::Decimal>,
    /// Sum of Regional Energy balance violations
    pub totalareagenviolation: Option<rust_decimal::Decimal>,
    /// Sum of Interconnector violations of standing data limits
    pub totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    /// Sum of Generic Constraint violations
    pub totalgenericviolation: Option<rust_decimal::Decimal>,
    /// Sum of Unit Ramp Rate violations
    pub totalramprateviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit capacity violations
    pub totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 5 min FCAS violations
    pub total5minviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional regulation FCAS violations
    pub totalregviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 6 sec FCAS violations
    pub total6secviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 60 sec FCAS violations
    pub total60secviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit energy constrained violations
    pub totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit offer violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit FCAS profile offer violations
    pub totalasprofileviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit Fast start profile violations
    pub totalfaststartviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if this Predispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run. This field has a default value of 0 and is not nullable
    pub intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minCasesolution2 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("CASESOLUTION".into()),
                        version: 2,
                    }
                    
    }
}
/// # Summary
/// 
/// ## P5MIN_PRICESENSITIVITIES
///  _Price Sensitivies for 5MinPD solution. New solution every 5 minutes. Current Scenarios defined in P5MIN_SCENARIODEMANDTRK/P5MIN_SCENARIODEMAND_
/// 
/// * Data Set Name: P5min
/// * File Name: Pricesensitivities
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minPricesensitivities1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Region
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Whether an Intervention constraint was defined in this run
    pub intervention: rust_decimal::Decimal,
    /// Flag to indicate if this period has an active intervention constraint: 0= No, 1= Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 1
    pub rrp1: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 2
    pub rrp2: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 3
    pub rrp3: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 4
    pub rrp4: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 5
    pub rrp5: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 6
    pub rrp6: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 7
    pub rrp7: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 8
    pub rrp8: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 9
    pub rrp9: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 10
    pub rrp10: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 11
    pub rrp11: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 12
    pub rrp12: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 13
    pub rrp13: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 14
    pub rrp14: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 15
    pub rrp15: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 16
    pub rrp16: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 17
    pub rrp17: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 18
    pub rrp18: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 19
    pub rrp19: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 20
    pub rrp20: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 21
    pub rrp21: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 22
    pub rrp22: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 23
    pub rrp23: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 24
    pub rrp24: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 25
    pub rrp25: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 26
    pub rrp26: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 27
    pub rrp27: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 28
    pub rrp28: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 29
    pub rrp29: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 30
    pub rrp30: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 31
    pub rrp31: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 32
    pub rrp32: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 33
    pub rrp33: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 34
    pub rrp34: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 35
    pub rrp35: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 36
    pub rrp36: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 37
    pub rrp37: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 38
    pub rrp38: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 39
    pub rrp39: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 40
    pub rrp40: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 41
    pub rrp41: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 42
    pub rrp42: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 43
    pub rrp43: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for P5minPricesensitivities1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("PRICESENSITIVITIES".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## P5MIN_CONSTRAINTSOLUTION
///  _The Five-Minute Pre-Dispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The Five-Minute Pre-dispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_CONSTRAINTSOLUTION shows binding and violated constraint results from the capacity evaluation, including the RHS value.<br>_
/// 
/// * Data Set Name: P5min
/// * File Name: Constraintsolution
/// * Data Version: 6
/// 
/// # Description
///  P5MIN_CONSTRAINTSOLUTION is public data, so is available to all participants. Source P5MIN_CONSTRAINTSOLUTION updates every five minutes. Volume Rows per day: ~2.3 million
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private &amp; Public
/// 
/// # Primary Key Columns
/// 
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minConstraintsolution6 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
    /// Right Hand Side value in the capacity evaluation
    pub rhs: Option<rust_decimal::Decimal>,
    /// Marginal cost of constraint (&gt;0 if binding)
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Amount of Violation (&gt;0 if  violating)
    pub violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// DUID to which the Constraint is confidential. Null denotes non-confidential
    pub duid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconid_effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of the Generic Constraint (ConstraintID). This field is used to track the version of this generic constraint applied in this dispatch interval
    pub genconid_versionno: Option<rust_decimal::Decimal>,
    /// Aggregation of the constraints LHS term solution values
    pub lhs: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run(INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    pub intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minConstraintsolution6 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "P5MIN".into(),
                        table_name: Some("CONSTRAINTSOLUTION".into()),
                        version: 6,
                    }
                    
    }
}
