#########
Tutorials
#########

*******************************************
Show current position from demo.signalk.org
*******************************************

In this tutorial we will make a sample client
that connects to the SignalK demo server using
the REST Api and shows the current position of
the self target.

This tutorial assumes that you have cargo installed and working
on the command line, a nice tutorial on the first steps in cargo
can be found in the `cargo book <https://doc.rust-lang.org/cargo/index.html>`_.

This tutorial so far have only been tested on Ubuntu (22.04 Jammy).

Create your signalk project
===========================

Prepare your development host for ssl-library content:

.. code-block:: sh

    sudo apt install libssl-dev


Now we can set up a new rust project with the cargo command

.. code-block:: sh

    cargo new sk_position
    cd sk_position

This will crate a sample application. Cargo will make a sample
hello_world application in main.rs. We will soon change that
into an application that connects to SignalKs demo server
and shows the position of the self ship.

Nest we need to add dependencies to the rust dependencies that we
need in our code.

* signalk - The SignalK data library
* reqwest - A http (REST) client library
* tokio - The async driver to make the async REST calls

.. code-block:: bash

    cargo add signalk
    cargo add reqwest -F json
    cargo add tokio -F full

Now we are ready to make out first API call to the demo-server.

Update the main.rs file to contain the following code:

.. code-block:: rust

    use signalk::V1FullFormat;
    use reqwest::Error;


    #[tokio::main]
    async fn main() -> Result<(), Error> {
        let api_url = "https://demo.signalk.org/signalk/v1/api/";
        println!("Connect and get data from: {}", api_url);
        let response = reqwest::get(api_url).await?;
        let sk_data: V1FullFormat = response.json().await?;
        if let Some(self_vessel) = sk_data.get_self() {
            if let Some(ref nav) = self_vessel.navigation {
                if let Some(ref pos) = nav.position {
                    print!("Position: lat {} long {}",
                           pos.value.latitude, pos.value.longitude);
                }
            }
        }
        Ok(())
    }

We can now run the application towards the demo.api and see the result:

.. code-block:: sh

    cargo run

