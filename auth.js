const fetch = require("node-fetch")

const noLoginPrefixes = [
    '/auth/login',
    '/image'
]

function noLoginRequired(url) {
    return noLoginPrefixes.some((prefix) => {
        return url.startsWith(prefix)
    })
}

class AuthService {
    constructor(auth_url) {
        this.auth_url = auth_url
    }

    login(req, res) {
        const username = req.query.username
        const password = req.query.password

        if (!username || !password) {
            res.sendStatus(400).end()
            return
        }

        return this.create_jwt(username, password)
            .then((jwt) => {
                console.info('successfully logged in ' + username, jwt)
                res.send(jwt)
            })
            .catch((e) => {
                console.warn('login failed for user <' + username + '>: ', e)
                res.sendStatus(400).end()
            })
    }

    check_logged_in(req, res, next) {
        console.info(req.url)
        if (typeof req.url !== 'undefined' && noLoginRequired(req.url)) {
            console.debug('no login required for ' + req.url)
            return next()
        }

        const token = req.get('Authorization')
        if (typeof token === 'undefined' || !token.substr(0, 'bearer'.length) === 'bearer') {
            console.debug('no auth headers set -> answering with HTTP 401')
            res.sendStatus(401).end()
            return
        }
        const jwt = token.substr('bearer '.length)
        console.debug('got jwt token ' + jwt)
        return this.verify_jwt(jwt)
            .then(() => next())
            .catch(() => {
                console.error('got invalid token ' + jwt)
                res.sendStatus(401).end()
            })
    }

    create_jwt(username, password) {
        const url = this.auth_url
            + '/authenticate?username='
            + encodeURIComponent(username)
            + '&password='
            + encodeURIComponent(password)
        return fetch(url, {
            method: 'POST'
        }).then((res) => {
            if (res.status === 200) {
                return Promise.resolve(res.text())
            } else {
                console.warn('auth service returned HTTP ' + resp.status + ' code')
                return Promise.reject()
            }
        })
    }

    verify_jwt(jwt) {
        return fetch(this.auth_url + '/verify/' + jwt, {
            method: 'POST'
        }).then((res) => {
            if (res.status === 200) {
                console.debug('jwt is valid')
                return Promise.resolve()
            } else {
                console.debug('jwt is invalid')
                return Promise.reject()
            }
        })
    }
}

module.exports = AuthService
