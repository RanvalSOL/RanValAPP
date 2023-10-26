const express = require("express");
const cors = require("cors");
const app = express();
app.use(cors());

const {
  generateMeta,
  generateImage,
} = require("./controllers/openaiController");

// app setup

app.listen(4000, () => console.log("listening to requests on port 4000"));

// middleware
app.use(express.json());
app.use(express.static("public"));

// routes
app.post("/openai/meta", generateMeta);
app.post("/openai/image", generateImage);
