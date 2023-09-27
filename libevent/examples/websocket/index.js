let keanu = new WebSocket("ws://localhost:3030/ws");
let denzel = new WebSocket("ws://localhost:3030/ws");

/* KEANU */

keanu.onopen = function (e) {
  console.log("[Open]");
  keanu.send("/name Keanu");
  keanu.send("Hi, I'm Keanu");
  setTimeout(function() {
    keanu.send("Bye, Denzel");
  }, 2000);
};

denzel.onopen = function (e) {
  console.log("[Open]");
  denzel.send("/name Denzel");
  denzel.send("Hi, I'm Denzel");
  setTimeout(function() {
    denzel.send("Bye, Keanu");
    denzel.send("/quit");
  }, 1000);
};

keanu.onmessage = function (event) {
  console.log(`[Message] ${event.data}`);
  // keanu.close(1000, "[Close]");
};

keanu.onclose = function (event) {
  if (event.wasClean) {
    console.log(`[Closed] code=${event.code} reason=${event.reason}`);
  } else {
    console.log(`[Dead] code=${event.code} reason=${event.reason}`);
  }
};

keanu.onerror = function (error) {
  console.log(`[Error]`);
};


/* DENZEL */

denzel.onmessage = function (event) {
  console.log(`[Message] ${event.data}`);
  // denzel.close(1000, "[Close]");
};

denzel.onclose = function (event) {
  if (event.wasClean) {
    console.log(`[Closed] code=${event.code} reason=${event.reason}`);
  } else {
    console.log(`[Dead] code=${event.code} reason=${event.reason}`);
  }
};

denzel.onerror = function (error) {
  console.log(`[Error]`);
};
