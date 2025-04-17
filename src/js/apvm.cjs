if (process.env.APVM_PATH) {
  let path = require("child_process")
    .execSync(`apvm info resolve {{ specifier }}`)
    .toString()
    .trim();
  module.exports = require(path);
} else {
  module.exports = require("{{ original }}");
}
