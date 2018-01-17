const fetch = require('node-fetch')

class StoreClient {

    constructor(store_url) {
        this.store_url = store_url
    }

    get_image(req, res) {
        console.info('loading image', req.params.image_id)
        return fetch(this.store_url + '/image/' + req.params.image_id)
            .then((thumb_res) => {
                res.status(thumb_res.status)
                return thumb_res.body.pipe(res)
            })
            .catch((e) => {
                console.error('error connecting to store service: ', e)
                res.sendStatus(500).end()
            })
    }

}

module.exports = StoreClient
