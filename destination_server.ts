Deno.serve(async (request) => {
	console.log([...request.headers.entries()])
	console.log(await request.text())

	return new Response('ok')
})
