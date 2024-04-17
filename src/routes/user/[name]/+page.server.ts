import { error } from "@sveltejs/kit";
import posts from "../../../assets/posts.json";

export function load({ params }) {
  const author_posts = posts.filter((post) => post.author === params.name);

  if (!author_posts.length) throw error(404);

  return {
    posts: author_posts,
    author: params.name,
  };
}
