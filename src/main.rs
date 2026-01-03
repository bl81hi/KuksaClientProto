use kuksa::val::v2::val_client::ValClient;
use kuksa::val::v2::GetValueRequest;
use kuksa::val::v2::GetValuesRequest;
use kuksa::val::v2::SignalId; 
use kuksa::val::v2::signal_id::Signal;

// Include the generated Kuksa modules
pub mod kuksa {
    pub mod val {
        pub mod v2 {
            tonic::include_proto!("kuksa.val.v2");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Kuksa gRPC server
    let addr = "http://127.0.0.1:55556";
    let mut client = ValClient::connect(addr).await?;
    println!("Connected to KUKSA VAL v2 Broker at address {}", addr);

    // Create a single value request - Vehicle.Speed
    let request = GetValueRequest {
        signal_id: Some(SignalId {
            signal: Some(Signal::Path("Vehicle.Speed".to_string())),
        }),
    };

    match client.get_value(request).await {
        Ok(response) => {
            let reply = response.into_inner();
            if let Some(dp) = reply.data_point {
                println!("Success! Received data point {:?}", dp.value);
            }
        },
        Err(e) => println!("Error: {}", e),
    }

    // Now we create a request for several Signals in one call
    let signals = vec![
        SignalId {
            signal: Some(Signal::Path("Vehicle.Speed".to_string())),
        },
        SignalId {
            signal: Some(Signal::Path("Vehicle.Chassis.Axle.Row1.Wheel.Left.Tire.Pressure".to_string())),
        },
    ];

    let request = GetValuesRequest {
        signal_ids: signals,
    };

    match client.get_values(request).await {
        Ok(response) => {
            let reply = response.into_inner();
            for (index, dp) in reply.data_points.iter().enumerate() {
                if let Some(val) = &dp.value {
                    println!("Value #{}: {:?}", index, val);
                } else {
                    println!("Value #{}: Datapoint has no value set", index);
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}



