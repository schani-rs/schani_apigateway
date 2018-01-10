const fetch = require("node-fetch");

class AuthService {
    constructor(auth_url) {
        this.auth_url = auth_url;
    }

    login(req, res) {
        const username = req.query.username;
        const password = req.query.password;

        if (!username || !password) {
            res.sendStatus(400).end();
            return;
        }

        return this.create_jwt(username, password)
            .then((jwt) => {
                console.info('successfully logged in ' + username, jwt);
                res.send(jwt);
            })
            .catch(() => {
                res.sendStatus(400).end();
            });
    }

    check_logged_in(req, res, next) {
        if (req.url.startsWith('/auth/login')) {
            next();
            return;
        }

        const token = req.get('Authorization');
        if (!token || !token.substr(0, 'bearer'.length) === 'bearer') {
            res.sendStatus(401).end();

        }
        const jwt = token.substr('bearer '.length);
        console.debug('got jwt token ' + jwt);
        return this.verify_jwt(jwt)
            .then(() => next())
            .catch(() => {
                console.error('got invalid token ' + jwt);
                res.sendStatus(401).end()
            });
    }

    create_jwt(username, password) {
        const url = this.auth_url
            + '/authenticate?username='
            + encodeURIComponent(username)
            + '&password='
            + encodeURIComponent(password);
        return fetch(url, {
            method: 'POST'
        }).then((res) => {
            if (res.status === 200) {
                return Promise.resolve(res.text());
            } else {
                return Promise.reject();
            }
        });
    }

    verify_jwt(jwt) {
        return fetch(this.auth_url + '/verify/' + jwt, {
            method: 'POST'
        }).then((res) => {
            if (res.status === 200) {
                console.debug('jwt is valid');
                return Promise.resolve();
            } else {
                console.debug('jwt is invalid');
                return Promise.reject();
            }
        });
    }
}

module.exports = AuthService;
