<script lang="ts">
    import { page } from "$app/stores";
    import { user_id } from "../stores/user.js";
</script>

<a class="button enabled" href="/colorwheel"> spin! </a>

<style>
    @import "./button_style.css";

    .button {
        right: 2%;
        top: 10%;
        border: none;
    }
</style>
