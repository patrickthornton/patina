<script lang="ts">
    import { colorFromHue } from "./hue.svelte";
    import User from "./interfaces.svelte";

    export let user: User;
</script>

<div class="user" style="background-color: {colorFromHue(user.hue)};">
    <div class="content">
        <a class="text" href="/user/{user.name}">{user.name}</a>
    </div>
</div>

<style>
    .user {
        display: flex;
        flex-direction: column;
        justify-content: space-around;
        align-items: center;
        padding-left: 1rem;
        padding-right: 1rem;
        padding-top: 0.5rem;
        padding-bottom: 0.2rem;
    }

    .content {
        width: 45%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        padding: 1rem;
    }

    .text {
        color: white;
        font-size: 1.5rem;
        font-weight: 300;
        line-height: 1.3;
        text-decoration: none;
    }
</style>
