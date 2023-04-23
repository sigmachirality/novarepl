// packages import
const express = require("express");
const app = express();
const cors = require("cors");
const axios = require("axios");
// enable CORS
app.use(cors());
// set the port on which our app wil run
// important to read from environment variable if deploying
const port = process.env.PORT || 5001;

// the route we're working with
app.get("/github", (req, res) => {
  // replace with a custom URL as required
  axios.post(`https://github.com/login/oauth/access_token`,
    {
      client_id: '495ee2906bb856b3022e',
      client_secret: '',
      code: req.query.code,
    },
    {
      headers: {
          Accept: "application/json"
      }
    }
  ).then(response => res.send(response.data));
  console.log(req.query.code);
});

// console text when app is running
app.listen(port, () => {
  console.log(`Server listening at http://localhost:${port}`);
});