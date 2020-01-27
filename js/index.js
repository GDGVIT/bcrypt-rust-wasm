import("../pkg/index.js")
  .then(module => console.log(module.Bcrypt.default().hashSync("password")))
  .catch(console.error);
