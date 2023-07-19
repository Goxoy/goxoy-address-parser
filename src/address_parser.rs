#[derive(Clone, Copy, Debug, PartialEq,Eq,Ord,PartialOrd)]
pub enum IPAddressVersion{
    IpV4,
    IpV6
}
#[derive(Clone, Copy, Debug, PartialEq,Eq,Ord,PartialOrd)]
pub enum ProtocolType{
    TCP,
    UDP
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddressParser {
    pub ip_address:String,
    pub port_no:usize,
    pub protocol_type:ProtocolType,
    pub ip_version:IPAddressVersion
}

impl AddressParser {
    pub fn new() -> Self {
        AddressParser{
            ip_address:String::new(),
            port_no:0,
            protocol_type:ProtocolType::TCP,
            ip_version:IPAddressVersion::IpV4,
        }
    }
    fn convert_string(&self,address_object:AddressParser)->String{
        let mut result_str=String::from("/ipv4/");
        if address_object.ip_version==IPAddressVersion::IpV6 {
            result_str.push_str("/ipv6/");
        }
        result_str.push_str(&address_object.ip_address);
        if address_object.protocol_type==ProtocolType::TCP {
            result_str.push_str("/tcp/");
        }else{
            result_str.push_str("/udp/");
        }
        result_str.push_str(&address_object.port_no.to_string());
        result_str
    }
    pub fn to_string(&self)->String{
        self.convert_string(self.to_object())
    }
    /*
    pub fn from_string(&self,address_string:String)->AddressParser{
        //self.convert_string(address_object)
    }
    */
    pub fn to_object(&self)->AddressParser{
        AddressParser{
            ip_address:self.ip_address.clone(),
            port_no:self.port_no,
            protocol_type:self.protocol_type,
            ip_version:self.ip_version,
        }
    }
    pub fn set_ip_version(&mut self,ip_version:IPAddressVersion){
        self.ip_version=ip_version;
    }
    pub fn set_ip(&mut self,ip_address:String){
        self.ip_address=ip_address;
    }
    pub fn set_port(&mut self,port_no:usize){
        self.port_no=port_no;
    }
    pub fn set_protocol(&mut self,protocol_type:ProtocolType){
        self.protocol_type=protocol_type;
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    let mut address_obj=AddressParser::new();
    address_obj.set_ip_version(IPAddressVersion::IpV4);
    address_obj.set_port(1234);
    address_obj.set_protocol(ProtocolType::TCP);
    address_obj.set_ip(String::from("123.456.789.456"));
    let result=address_obj.to_string();
    println!("{}",result);
    assert_eq!(true,true)
}
