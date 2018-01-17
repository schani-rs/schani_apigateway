const express = require('express')

const AuthService = require('./auth')
const ImportClient = require('./import_client')
const StoreClient = require('./store_client')
const ThumbnailClient = require('./thumbnail_client')

const app = express()
const auth = new AuthService('http://auth:8000')
const import_client = new ImportClient('http://import:8000')
const store_client = new StoreClient('http://store:8000')
const thumbnail_client = new ThumbnailClient('http://thumbnailer-cache:80')

app.use(auth.check_logged_in.bind(auth))

// Auth
app.post('/auth/login', auth.login.bind(auth))

// Import service
app.post('/imports', (req, res) => import_client.start_import(req, res))
app.put('/imports/:import_id/raw', (req, res) => import_client.upload_raw_file(req, res))
app.put('/imports/:import_id/sidecar', (req, res) => import_client.upload_sidecar(req, res))
app.put('/imports/:import_id/image', (req, res) => import_client.upload_image(req, res))
app.post('/imports/:import_id/finish', (req, res) => import_client.finish_import(req, res))

// Images and thumbnails
app.get('/image/:image_id', (req, res) => store_client.get_image(req, res))
app.get('/image/:image_id/thumb', (req, res) => thumbnail_client.get_thumbnail(req, res))

app.listen(3000, () => console.log('Schani API gateways listening on port 3000'))
