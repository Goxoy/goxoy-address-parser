#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum IPAddressVersion{
    IpV4,
    IpV6
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum ProtocolType{
    TCP,
    UDP,
    WEBSOCKET
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct AddressParser {
    pub ip_address:String,
    pub port_no:usize,
    pub protocol_type:ProtocolType,
    pub ip_version:IPAddressVersion
}

impl AddressParser {
    pub fn object_to_string(address_object:AddressParser)->String{
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
    pub fn string_to_object(address_string:String)->AddressParser{
        let tmp_arr=address_string.split("/");
        let mut tmp_type=ProtocolType::TCP;
        let mut tmp_ip_ver=IPAddressVersion::IpV4;
        let mut tmp_ip_addr=String::from("0.0.0.0");
        let mut tmp_port_no=usize::MIN;
        let mut started=false;
        let mut count=0;
        for part in tmp_arr {
            if started==true{
                if count==0{
                    tmp_ip_addr=part.to_string();
                }
                if count==1{
                    if part.eq("tcp"){
                        tmp_type=ProtocolType::TCP;
                    }
                    if part.eq("udp"){
                        tmp_type=ProtocolType::UDP;
                    }
                }
                if count==2{
                    tmp_port_no=part.parse().unwrap_or_default();
                }
                count=count+1;
            }
            if part.eq("ipv4"){
                tmp_ip_ver=IPAddressVersion::IpV4;
                started=true;
            }
            if part.eq("ipv6"){
                tmp_ip_ver=IPAddressVersion::IpV6;
                started=true;
            }
        }        
        AddressParser{
            ip_address:tmp_ip_addr,
            port_no:tmp_port_no,
            protocol_type:tmp_type,
            ip_version:tmp_ip_ver,
        }
    }
    pub fn binding_addr_to_object(local_addr:String, protocol_type:ProtocolType, ip_version:IPAddressVersion)->AddressParser{
        let collection = local_addr.split(".").collect::<Vec<&str>>();
        AddressParser{
            ip_address: collection[0].to_string(),
            port_no: collection[1].parse::<usize>().unwrap(),
            protocol_type,
            ip_version,
        }
    }
    pub fn binding_addr_to_string(local_addr:String, protocol_type:ProtocolType, ip_version:IPAddressVersion)->String{
        AddressParser::object_to_string(
            AddressParser::binding_addr_to_object(
                local_addr, 
                protocol_type, 
                ip_version
            )
        )
    }
    pub fn local_addr_for_binding(address_object:AddressParser)->String{
        let mut bind_str = address_object.ip_address;
        bind_str.push_str(":");
        bind_str.push_str(&address_object.port_no.to_string());
        bind_str
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    
    let addr_obj=AddressParser{
        ip_version: IPAddressVersion::IpV4,
        ip_address:"127.0.0.1".to_string(),
        port_no:1234,
        protocol_type:ProtocolType::TCP
    };
    let addr_str=AddressParser::object_to_string(addr_obj.clone());
    
    //let local_addr_for_socket=AddressParser::local_addr_for_binding(addr_obj.clone());
    //println!("local_addr_for_socket: {}",local_addr_for_socket.clone());

    let convert_obj=AddressParser::string_to_object(addr_str.clone());

    if addr_obj.eq(&convert_obj.clone()){
        assert_eq!(true,true)
    }else{
        assert_eq!(false,true)
    }
}

