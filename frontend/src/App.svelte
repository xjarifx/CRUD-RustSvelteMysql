<script>
    import { onMount } from 'svelte';
    import axios from 'axios';

    let users = [];
    let name = '';
    let email = '';
    let selectedUser = null;

    async function fetchUsers() {
        try {
            const response = await axios.get('http://localhost:8080/users');
            users = response.data;
        } catch (error) {
            console.error('Error fetching users:', error);
        }
    }

    async function createUser() {
        try {
            await axios.post('http://localhost:8080/users', { name, email });
            name = '';
            email = '';
            fetchUsers();
        } catch (error) {
            console.error('Error creating user:', error);
        }
    }

    async function updateUser() {
        try {
            await axios.put(`http://localhost:8080/users/${selectedUser.id}`, {
                name: selectedUser.name,
                email: selectedUser.email
            });
            selectedUser = null;
            fetchUsers();
        } catch (error) {
            console.error('Error updating user:', error);
        }
    }

    async function deleteUser(id) {
        try {
            await axios.delete(`http://localhost:8080/users/${id}`);
            fetchUsers();
        } catch (error) {
            console.error('Error deleting user:', error);
        }
    }

    onMount(fetchUsers);
</script>

<div class="container">
    <h1>User Management</h1>

    <div class="form-group">
        <input
            bind:value={name}
            placeholder="Name"
            class="input-field"
            type="text"
        />
        <input
            bind:value={email}
            placeholder="Email"
            class="input-field"
            type="email"
        />
        <button on:click={createUser} class="button">Add User</button>
    </div>

    <div class="table-container">
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Name</th>
                    <th>Email</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {#each users as user}
                    <tr>
                        <td>{user.id}</td>
                        <td>{user.name}</td>
                        <td>{user.email}</td>
                        <td>
                            <button on:click={() => selectedUser = user} class="button edit">Edit</button>
                            <button on:click={() => deleteUser(user.id)} class="button delete">Delete</button>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>

    {#if selectedUser}
        <div class="edit-form">
            <h2>Edit User</h2>
            <input
                bind:value={selectedUser.name}
                placeholder="Name"
                class="input-field"
            />
            <input
                bind:value={selectedUser.email}
                placeholder="Email"
                class="input-field"
            />
            <button on:click={updateUser} class="button">Save Changes</button>
        </div>
    {/if}
</div>

<style>
    .container {
        max-width: 800px;
        margin: 0 auto;
        padding: 20px;
    }

    .form-group {
        margin-bottom: 20px;
    }

    .input-field {
        margin: 5px;
        padding: 8px;
        border: 1px solid #ddd;
        border-radius: 4px;
    }

    .button {
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        margin: 5px;
    }

    .button.edit {
        background-color: #4CAF50;
        color: white;
    }

    .button.delete {
        background-color: #f44336;
        color: white;
    }

    .table-container {
        margin-top: 20px;
    }

    table {
        width: 100%;
        border-collapse: collapse;
    }

    th, td {
        padding: 12px;
        text-align: left;
        border-bottom: 1px solid #ddd;
    }

    th {
        background-color: #f5f5f5;
    }

    tr:hover {
        background-color: #f5f5f5;
    }

    .edit-form {
        margin-top: 20px;
        padding: 20px;
        background-color: #f9f9f9;
        border-radius: 8px;
    }
</style>
