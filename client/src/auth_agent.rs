use yew::worker::*;

struct Worker {
  link: AgentLink<Worker>
}


pub enum AuthRequest {
  IsAuthorized(String)
}

pub enum AuthResponse {
  IsAuthorized(bool)
}

impl Agent for Worker {
  type Reach = Context;
  type Message = Msg;
  type Input = AuthRequest;
  type Output AuthResponse;

  fn create(link: AgentLink<Self>) -> Self {
    Worker {
      link
    }
  }

  fn update(&mut self, msg: Self::Message) {
    match msg {
      // Here is where you would match messages the Worker sends itself
      _ => {}
    }
  }

  fn handle(&mut self, msg: Self::Input, who: HandlerId) {
    match msg {
      // Here is where you handle messages from other components
      AuthRequest(jwt) => {
        // Do whatever is needed to determine if the JWT is valid
        let auth_result = true;
        // Now we can send a response
        // The self.link is the Agent, which will be in a background thread. We know who sent us the message because of the HandlerId, so we can just direct the response back to them
        self.link.response(who, Response::AuthResponse::IsAuthorized(auth_result))
      }
    }
  }
}

// Now we need something to spawn the worker in the web browser world
struct AuthModel {
  context: Box<Bridge<context::Worker>>
}

enum Msg {
  ContextMsg(context::Response)
}

impl Component for AuthModel {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.send_back(|_| Msg::ContextMsg);
    let context = context::Worker::bridge(callback)
    AuthModel { context }
  }
}


