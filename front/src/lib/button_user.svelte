<script lang="ts">
    import { user_id, user_name } from "../stores/user.js";
</script>

{#if $user_id != 0}
    <a class="button enabled" href="/user/{$user_name}"> you </a>
{:else}
    <div class="button disabled">log in!</div>
{/if}

<style>
    @import "./button_style.css";

    .button {
        right: 20%;
        top: 60%;
    }
</style>
