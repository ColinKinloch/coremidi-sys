extern crate coremidi_sys as cm;
extern crate core_foundation;

use core_foundation::base::TCFType;
use core_foundation::propertylist::CFPropertyListRef;
use core_foundation::string::{CFString, CFStringRef};

fn get_string_property(device: cm::MIDIObjectRef, key: CFStringRef) -> Result<CFString, cm::OSStatus> {
    let mut string_ref = CFString::new("").as_concrete_TypeRef();
    let result = unsafe { cm::MIDIObjectGetStringProperty(device, key, &mut string_ref as *mut CFStringRef) };
    if result == 0 {
      Ok(unsafe { CFString::wrap_under_get_rule(string_ref) })
    } else {
      Err(result)
    }
}

fn main() {
    let device_count = unsafe { cm::MIDIGetNumberOfDevices() };
    for i in 0..device_count {
      let device = unsafe { cm::MIDIGetDevice(i) };
      //let mut device_properties: CFPropertyListRef = unsafe { mem::uninitialized() };
      //let result = unsafe { cm::MIDIObjectGetProperties(device, &mut device_properties, 0) };
      let device_name = get_string_property(device, unsafe { cm::kMIDIPropertyName }).unwrap();
      let device_manu = get_string_property(device, unsafe { cm::kMIDIPropertyManufacturer }).unwrap();
      println!("Device: {}; {}, {}", i, device_name, device_manu);
      let entity_count = unsafe { cm::MIDIDeviceGetNumberOfEntities(device) };
      for j in 0..entity_count {
        let entity = unsafe { cm::MIDIDeviceGetEntity(device, j) };
        let entity_name = get_string_property(entity, unsafe { cm::kMIDIPropertyName }).unwrap();
        println!("  Entity: {}; {}", j, entity_name);
        let source_count = unsafe { cm::MIDIEntityGetNumberOfSources(entity) };
        for k in 0..source_count {
          let source = unsafe { cm::MIDIEntityGetSource(entity, k) };
          let source_name = get_string_property(source, unsafe { cm::kMIDIPropertyDisplayName }).unwrap();
          println!("    Source: {}; {}", k, source_name);
        }
        let destination_count = unsafe { cm::MIDIEntityGetNumberOfDestinations(entity) };
        for k in 0..destination_count {
          let destination = unsafe { cm::MIDIEntityGetDestination(entity, k) };
          let destination_name = get_string_property(destination, unsafe { cm::kMIDIPropertyDisplayName }).unwrap();
          println!("    Destination: {}; {}", k, destination_name);
        }
      }
    }
}
