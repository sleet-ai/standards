// ====================================
async function ping() {
  const res = await fetch("https://api.github.com/zen");
  const text = await res.text();
  console.log("===========================");
  console.log("fetch() - https://api.github.com/zen");
  console.log(text);
}
// ====================================
ping();
