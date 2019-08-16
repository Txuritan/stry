const Fastify = require('fastify')({ logger: true })
const Turndown = require("turndown");

const Service = new Turndown();

Fastify.post("/", (req, res) => {
    res.send(Service.turndown(req.body));
});

Fastify.listen(8902);
