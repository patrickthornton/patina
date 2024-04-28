<script lang="ts">
    import { user_name } from "../stores/user.js";
</script>

<a class="button-user" href="/user/{$user_name}"> you </a>

<style>
    .button-user {
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        position: fixed;
        right: 20%;
        top: 60%;
        color: white;
        border-radius: 1rem;
        padding: 1rem;
        border: 1px solid white;
        width: 4rem;
        text-decoration: none;
    }
</style>
