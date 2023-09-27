let keanu = new WebSocket("ws://localhost:3030/ws");
let denzel = new WebSocket("ws://localhost:3030/ws");

/* KEANU */

keanu.onopen = function (e) {
  console.log("[Keanu Open]");
  keanu.send("Hello?");
  keanu.send("/name Keanu");
  keanu.send("Hi, I'm Keanu");
  setTimeout(function() {
    keanu.send("Bye, Denzel");
  }, 2000);
};

denzel.onopen = function (e) {
  console.log("[Denzel Open]");
  denzel.send("/name Denzel");
  denzel.send("Hi, I'm Denzel");
  setTimeout(function() {
    denzel.send("Bye, Keanu");
    denzel.send("/quit");
  }, 1000);
};

keanu.onmessage = function (event) {
  console.log(`[Message] ${event.data}`);
  // keanu.close(1000, "[Keanu Close]");
};

keanu.onclose = function (event) {
  if (event.wasClean) {
    console.log(`[Keanu Closed] code=${event.code} reason=${event.reason}`);
  } else {
    console.log(`[Keanu Dead] code=${event.code} reason=${event.reason}`);
  }
};

keanu.onerror = function (error) {
  console.log(`[Keanu Error]`);
};


/* DENZEL */

denzel.onmessage = function (event) {
  // console.log(`[Message] ${event.data}`);
  // denzel.close(1000, "[Denzel Close]");
};

denzel.onclose = function (event) {
  if (event.wasClean) {
    console.log(`[Denzel Closed] code=${event.code} reason=${event.reason}`);
  } else {
    console.log(`[Denzel Dead] code=${event.code} reason=${event.reason}`);
  }
};

denzel.onerror = function (error) {
  console.log(`[Denzel Error]`);
};

