const fetch = require('node-fetch')

class ImportClient {

    constructor(import_url) {
        this.import_url = import_url
    }

    start_import(req, res) {
        console.info('starting import')
        return fetch(this.import_url + '/imports', {
            method: 'POST',
            body: req
        }).then((import_res) => {
            res.status(import_res.status)
            return import_res.body.pipe(res)
        }).catch((e) => {
            console.error('error connecting to import service: ', e)
            res.sendStatus(500).end()
        })
    }

    upload_raw_file(req, res) {
        console.info('starting raw file upload', req.params.import_id)
        return fetch(this.import_url + '/imports/' + req.params.import_id + '/raw', {
            method: 'PUT',
            body: req
        }).then((import_res) => {
            res.status(import_res.status)
            return import_res.body.pipe(res)
        }).catch((e) => {
            console.error('error connecting to import service: ', e)
            res.sendStatus(500).end()
        })
    }

    upload_sidecar(req, res) {
        console.info('starting sidecar upload', req.params.import_id)
        return fetch(this.import_url + '/imports/' + req.params.import_id + '/sidecar', {
            method: 'PUT',
            body: req
        }).then((import_res) => {
            res.status(import_res.status)
            return import_res.body.pipe(res)
        }).catch((e) => {
            console.error('error connecting to import service: ', e)
            res.sendStatus(500).end()
        })
    }

    upload_image(req, res) {
        console.info('starting image upload')
        return fetch(this.import_url + '/imports/' + req.params.import_id + '/image', {
            method: 'PUT',
            body: req
        }).then((import_res) => {
            res.status(import_res.status)
            return import_res.body.pipe(res)
        }).catch((e) => {
            console.error('error connecting to import service: ', e)
            res.sendStatus(500).end()
        })
    }

    finish_import(req, res) {
        console.info('finishing import')
        return fetch(this.import_url + '/imports/' + req.params.import_id + '/finish', {
            method: 'POST',
            body: req
        }).then((import_res) => {
            res.status(import_res.status)
            return import_res.body.pipe(res)
        }).catch((e) => {
            console.error('error connecting to import service: ', e)
            res.sendStatus(500).end()
        })
    }

}

module.exports = ImportClient
