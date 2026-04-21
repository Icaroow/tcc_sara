class Auth {
    static setUser(id, name) {
        localStorage.setItem('userId', id);
        localStorage.setItem('userName', name);
        localStorage.setItem('token', 'bearer_' + id);
    }

    static getUser() {
        return {
            id: localStorage.getItem('userId'),
            name: localStorage.getItem('userName'),
        };
    }

    static isLoggedIn() {
        return !!localStorage.getItem('userId');
    }

    static logout() {
        localStorage.removeItem('userId');
        localStorage.removeItem('userName');
        localStorage.removeItem('token');
        window.location.href = 'login.html';
    }

    static redirectIfNotLoggedIn() {
        if (!this.isLoggedIn()) {
            window.location.href = 'login.html';
        }
    }
}