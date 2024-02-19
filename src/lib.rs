use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

pub fn poc() {
    let x = Python::with_gil(|py| {
        let platform = PyModule::import(py, "sonic_platform.platform").unwrap();
        let psus: &PyList = platform
            .call_method0("Platform").unwrap()
            .call_method0("get_chassis").unwrap()
            .call_method0("get_all_psus").unwrap()
            .extract::<&PyList>().unwrap();

        for psu in psus {
            let voltage: f32 = psu.call_method0("get_voltage").unwrap().extract::<f32>().unwrap();
            println!("{voltage:#?}");
        };
    });
}

struct Fan {
    pub eeprom: String,
    pub fan_index: u8,
    pub fantray_index: u8,
    pub max_speed: u32,
    pub presence_reg: String,
    pub rpm_file: String,
    pub status_file: String,
}

impl Fan {
    pub fn get_direction() {
    }

    pub fn get_model() {

    }

    pub fn get_name() {

    }

    pub fn get_position_in_parent() {

    }

    pub fn get_presence() {

    }

    pub fn get_revision() {

    }

    pub fn get_serial() {

    }

    pub fn get_speed() {

    }

    pub fn get_speed_rpm() {

    }

    pub fn get_speed_tolerance() {

    }

    pub fn get_status() {

    }

    pub fn get_status_led() {

    }

    pub fn get_target_speed() {

    }

    pub fn is_over_speed() {

    }

    pub fn is_psu_fan() {

    }

    pub fn is_replaceable() {

    }

    pub fn is_under_speed() {

    }

    pub fn set_speed() {

    }

    pub fn set_status_led() {

    }
}


['DEVICE_TYPE', 'FAN_DIRECTION_EXHAUST', 'FAN_DIRECTION_INTAKE', 'FAN_DIRECTION_NOT_APPLICABLE', 'STATUS_LED_COLOR_AMBER', 'STATUS_LED_COLOR_GREEN', 'STATUS_LED_COLOR_OFF', 'STATUS_LED_COLOR_RED', '__class__', '__delattr__', '__dict__', '__dir__', '__doc__', '__eq__', '__format__', '__ge__', '__getattribute__', '__getstate__', '__gt__', '__hash__', '__init__', '__init_subclass__', '__le__', '__lt__', '__module__', '__ne__', '__new__', '__reduce__', '__reduce_ex__', '__repr__', '__setattr__', '__sizeof__', '__str__', '__subclasshook__', '__weakref__', '_get_cpld_register', 'dir_reg', 'eeprom', 'fan_index', 'fantray_index', 'get_direction', 'get_model', 'get_name', 'get_position_in_parent', 'get_presence', 'get_revision', 'get_serial', 'get_speed', 'get_speed_rpm', 'get_speed_tolerance', 'get_status', 'get_status_led', 'get_target_speed', 'is_over_speed', 'is_psu_fan', 'is_replaceable', 'is_under_speed', 'max_speed', 'presence_reg', 'rpm_file', 'set_speed', 'set_status_led', 'status_file']
>>> dir(c.get_all_fans()[0]['DEVICE_TYPE']
