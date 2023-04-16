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

.. code-block:: bash

    cargo new sk_position
    cd sk_position
    cargo add signalk

This will crate a sample application and add the dependency to
the signalk library. Next we need to add a library to let us connect
to the REST api and get some data. We will be using reqwest and tokio for this.

.. code-block:: bash

    cargo add reqwest -F json
    cargo add tokio -F full

Now we are ready to make out first API call to the demo-server.

Update the main.rs file to contain the following code:

.. code-block:: rust

    use signalk::signalk::full::V1FullFormat;
    use reqwest::Error;


    #[tokio::main]
    async fn main() -> Result<(), Error> {
        let api_url = "https://demo.signalk.org/signalk/v1/api/";
        println!("url: {}", api_url);

        let response = reqwest::get(api_url).await?;

        let sk_data: V1FullFormat = response.json().await?;

        println!("Got: {:?}", sk_data);
        Ok(())
    }

We can now run the application towards the demo.api and see the result:

.. code-block:: sh

    cargo run
