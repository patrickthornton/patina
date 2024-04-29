<script lang="ts">
    export let author: string;
</script>

<a class="button enabled" href="/user/{author}/liked"> likes </a>

<style>
    @import "./button_style.css";

    .button {
        position: absolute;
        right: 33%;
        top: 1.5%;
        border: none;
    }
</style>
