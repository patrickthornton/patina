<script lang="ts">
    import { page } from "$app/stores";
    import { user_id } from "../stores/user.js";

    $: onFollowing = $page.url.pathname === "/following";
</script>

{#if $user_id != 0}
    {#if onFollowing}
        <a class="button enabled" href="/"> gradient </a>
    {:else}
        <a class="button enabled" href="/following"> following </a>
    {/if}
{:else}
    <div class="button disabled">log in!</div>
{/if}

<style>
    @import "./button_style.css";

    .button {
        right: 20%;
        top: 40%;
    }
</style>
