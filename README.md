# Goxoy Address Parser

Soketler için adreslerin nesne ve string çevrimlerini yapan kitaplık

## Kullanım / Örnekler

```rust
    // port ve tür belirterek nesne oluşturabilirsiniz.
    let addr_obj = AddressParser{
        ip_version: IPAddressVersion::IpV4,
        ip_address: "127.0.0.1".to_string(),
        port_no: 1234,
        protocol_type: ProtocolType::TCP
    };

    // oluşturulan nesne ile string olarak dönüştürülebilir
    let addr_str = AddressParser::object_to_string(addr_obj.clone());
    println!("addr_str: {}", addr_str.clone());
    /*
    çıktısı
    addr_str: /ipv4/127.0.0.1/tcp/1234
    */

    // aşağıdaki fonksiyon ile string tekrardan nesneye çevrilir.
    let convert_obj = AddressParser::string_to_object(addr_str.clone());

    // bu fonksiyon ile TCP veya UDP soket için doğrudan string çıktısı alabilirsiniz.
    let local_addr_for_socket = AddressParser::local_addr_for_binding(addr_str.clone());
    println!("local_addr_for_socket: {}", local_addr_for_socket.clone());
    /*
    çıktısı
    local_addr_for_socket: 127.0.0.1:1234
    */
```

  
## Lisans

[MIT](https://choosealicense.com/licenses/mit/)