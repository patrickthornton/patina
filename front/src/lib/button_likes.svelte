<script lang="ts">
    export let name: String;
</script>

<a class="button enabled" href="/user/{name}/liked"> likes </a>

<style>
    @import "./button_style.css";

    .button {
        right: 33%;
        top: 1.5%;
        border: none;
    }
</style>
