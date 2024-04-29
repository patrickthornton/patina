<script lang="ts">
    export let user_name;
</script>

<a class="button enabled" href="/user/{$user_name}/liked"> likes </a>

<style>
    @import "./button_style.css";

    .button {
        right: 33%;
        top: 1.5%;
        border: none;
    }
</style>
