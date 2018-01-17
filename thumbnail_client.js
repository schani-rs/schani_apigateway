const fetch = require('node-fetch')

class ThumbnailClient {

    constructor(thumbnailer_url) {
        this.thumbnailer_url = thumbnailer_url
    }

    get_thumbnail(req, res) {
        console.info('loading thumbnail');
        return fetch(this.thumbnailer_url + '/thumb/' + req.params.image_id)
            .then((thumb_res) => {
                console.info('thumbnail' + req.params.image_id + ' loaded', thumb_res.status)
                res.status(thumb_res.status)
                return thumb_res.body.pipe(res)
            })
            .catch((e) => {
                console.error('error connecting to thumbnail service: ', e)
                return res.sendStatus(500).end()
            })
    }

}

module.exports = ThumbnailClient
