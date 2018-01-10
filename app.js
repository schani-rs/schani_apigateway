const express = require('express');

const AuthService = require('./auth');
const ImportClient = require('./import_client');

const app = express();
const auth = new AuthService('http://localhost:8005');
const import_client = new ImportClient('http://localhost:8001');

app.use(auth.check_logged_in.bind(auth));

// Auth
app.post('/auth/login', auth.login.bind(auth));

// Import service
app.post('/imports', (req, res) => import_client.start_import(req, res));
app.put('/imports/:import_id/raw', (req, res) => import_client.upload_raw_file(req, res));
app.put('/imports/:import_id/sidecar', (req, res) => import_client.upload_sidecar(req, res));
app.put('/imports/:import_id/image', (req, res) => import_client.upload_image(req, res));
app.post('/imports/:import_id', (req, res) => import_client.finish_import(req, res));

app.listen(3000, () => console.log('Schani API gateways listening on port 3000'));