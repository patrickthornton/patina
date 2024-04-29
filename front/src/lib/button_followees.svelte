<script lang="ts">
    export let author: string = "";
</script>

<a class="button enabled" href="/user/{author}/followees"> followees </a>

<style>
    @import "./button_style.css";

    .button {
        position: absolute;
        right: 40%;
        top: 1.5%;
        border: none;
        font-weight: normal;
    }
</style>
