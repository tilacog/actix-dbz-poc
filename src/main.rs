extern crate actix;
extern crate futures;
extern crate tokio;

use actix::prelude::*;
use futures::Future;

//
// Scouter actor and messages
//

struct Scouter;
impl Actor for Scouter {
    type Context = Context<Self>;
}

struct ProbeKi(usize);
impl Message for ProbeKi {
    type Result = usize;
}

impl Handler<ProbeKi> for Scouter {
    type Result = <ProbeKi as Message>::Result;
    fn handle(&mut self, ProbeKi(ki_level): ProbeKi, _: &mut Context<Self>) -> Self::Result {
        if ki_level >= 9_000 {
            println!("ITS OVER NINE THOUSAND!!!");
            System::current().stop();
        };
        return ki_level;
    }
}

//
// Fighter actors and messages
//

#[derive(Message)]
struct RaiseKi;

struct ZFighter {
    name: String,
    ki: usize,
    /// could this be moved to <Self as Actor>::Context?
    scouter: Addr<Scouter>,
}

impl Actor for ZFighter {
    type Context = Context<Self>;
}

impl Handler<RaiseKi> for ZFighter {
    type Result = <RaiseKi as Message>::Result;
    fn handle(&mut self, _msg: RaiseKi, _: &mut Context<Self>) -> Self::Result {
        self.ki += 1_000;
        println!("{} raised its ki to {}", self.name, self.ki);

        // send message to the probe/scouter actor with the new power level
        tokio::spawn(
            self.scouter
                .send(ProbeKi(self.ki))
                .map(|_res| {})
                .map_err(|_| ()),
        );
    }
}

fn main() {
    System::run(|| {
        println!("The battle begins.",);

        // start actors
        let scouter = Scouter.start();
        let goku = ZFighter {
            name: "kakaroto".into(),
            ki: 0,
            scouter: scouter.clone(),
        }.start();

        // send messages to make goku raise it's ki
        for _ in 1..10 {
            let res = goku.send(RaiseKi);
            tokio::spawn(res.map(|_res| {}).map_err(|_| ()));
        }
    });
}
