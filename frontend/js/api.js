const API_BASE_URL = 'http://127.0.0.1:3000';

class API {
    static getHeaders() {
        const token = localStorage.getItem('token');
        const headers = {
            'Content-Type': 'application/json',
        };
        if (token) {
            headers['Authorization'] = `Bearer ${token}`;
        }
        return headers;
    }

    // ===== USERS =====
    static async registerUser(name) {
        try {
            const response = await fetch(`${API_BASE_URL}/users`, {
                method: 'POST',
                headers: this.getHeaders(),
                body: JSON.stringify({ name }),
            });
            return await response.json();
        } catch (error) {
            console.error('Erro ao registrar:', error);
            throw error;
        }
    }

    static async getUsers() {
        try {
            const response = await fetch(`${API_BASE_URL}/users`, {
                headers: this.getHeaders(),
            });
            return await response.json();
        } catch (error) {
            console.error('Erro ao buscar usuários:', error);
            throw error;
        }
    }

    static async getUserById(id) {
        try {
            const response = await fetch(`${API_BASE_URL}/users/${id}`, {
                headers: this.getHeaders(),
            });
            return await response.json();
        } catch (error) {
            console.error('Erro ao buscar usuário:', error);
            throw error;
        }
    }

    // ===== PATRIMONIOS =====
    static async getPatrimonios() {
        try {
            const response = await fetch(`${API_BASE_URL}/patrimonios`, {
                headers: this.getHeaders(),
            });
            return await response.json();
        } catch (error) {
            console.error('Erro ao buscar patrimônios:', error);
            throw error;
        }
    }

    static async getPatrimoniosByUser(userId) {
        try {
            const response = await fetch(`${API_BASE_URL}/patrimonios/user/${userId}`, {
                headers: this.getHeaders(),
            });
            return await response.json();
        } catch (error) {
            console.error('Erro ao buscar patrimônios do usuário:', error);
            throw error;
        }
    }

    static async getPatrimonioById(id) {
        try {
            const response = await fetch(`${API_BASE_URL}/patrimonios/${id}`, {
                headers: this.getHeaders(),
            });
            return await response.json();
        } catch (error) {
            console.error('Erro ao buscar patrimônio:', error);
            throw error;
        }
    }

    static async createPatrimonio(data) {
        try {
            const response = await fetch(`${API_BASE_URL}/patrimonios`, {
                method: 'POST',
                headers: this.getHeaders(),
                body: JSON.stringify(data),
            });
            return await response.json();
        } catch (error) {
            console.error('Erro ao criar patrimônio:', error);
            throw error;
        }
    }

    static async updatePatrimonio(id, data) {
        try {
            const response = await fetch(`${API_BASE_URL}/patrimonios/${id}`, {
                method: 'PUT',
                headers: this.getHeaders(),
                body: JSON.stringify(data),
            });
            return await response.json();
        } catch (error) {
            console.error('Erro ao atualizar patrimônio:', error);
            throw error;
        }
    }

    static async deletePatrimonio(id) {
        try {
            const response = await fetch(`${API_BASE_URL}/patrimonios/${id}`, {
                method: 'DELETE',
                headers: this.getHeaders(),
            });
            return await response.text();
        } catch (error) {
            console.error('Erro ao deletar patrimônio:', error);
            throw error;
        }
    }

    static async getTotalValorPatrimonios(userId) {
        try {
            const response = await fetch(`${API_BASE_URL}/patrimonios/user/${userId}/total`, {
                headers: this.getHeaders(),
            });
            return await response.json();
        } catch (error) {
            console.error('Erro ao buscar total:', error);
            throw error;
        }
    }
}