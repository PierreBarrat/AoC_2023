### A Pluto.jl notebook ###
# v0.19.35

using Markdown
using InteractiveUtils

# ╔═╡ fea40d22-c40a-468d-b74c-fe1a0e6b505d
begin
	using Chain
end

# ╔═╡ 87be8544-979a-11ee-05ab-f78f386a848c
input = "input.txt"

# ╔═╡ 7eb57748-3ed4-4fc4-93d1-84ceb7adbfaf
data = let
	lines = readlines(input)
	map(lines) do l
		@chain split(l, " ") map(x -> parse(Int, x), _)
	end
end

# ╔═╡ 57614da3-3db1-4822-a630-7b6275853b63
function differentiate!(X, L)
	for i in 2:L
		X[i-1] = X[i] - X[i-1]
	end
	
	X[L:end] .= 0
	return nothing
end

# ╔═╡ 2d974ba2-2110-40cd-8c72-0a33123d5646


# ╔═╡ f2cf036f-e512-4517-89b8-402b40e39c71
function predict_next!(X, cnt)
	if all(==(0), X) return 0 end
	if cnt > 100 @warn "Did not converge!" X; return 0 end
	
	frst = X[1]
	# @info "before diff" X
	differentiate!(X, length(X) - cnt)
	# @info "After diff" X
	pred = predict_next!(X, cnt + 1)
	# @info "Predicting: $pred --> $(frst) - $(pred) = $(frst - pred)"
	return frst - pred
end

# ╔═╡ ef805a41-e34c-4a0f-815a-96b7df45751c


# ╔═╡ ba765d01-00e3-4b2e-9e90-446775dbeb67
predict_next!(copy(data[1]), 0)

# ╔═╡ fa2941f2-9c6a-4814-a0f4-242bc8c41914
let
	dat_c = deepcopy(data)
	# predict_next!(dat_c[1])
	rvals = map(dat_c) do X
		@info X
		r = predict_next!(X, 0)
		@info r
		r
	end
	sum(rvals)
end

# ╔═╡ 00000000-0000-0000-0000-000000000001
PLUTO_PROJECT_TOML_CONTENTS = """
[deps]
Chain = "8be319e6-bccf-4806-a6f7-6fae938471bc"

[compat]
Chain = "~0.5.0"
"""

# ╔═╡ 00000000-0000-0000-0000-000000000002
PLUTO_MANIFEST_TOML_CONTENTS = """
# This file is machine-generated - editing it directly is not advised

julia_version = "1.9.4"
manifest_format = "2.0"
project_hash = "4a0b84801f02292c3dc8d40d570055df7d829417"

[[deps.Chain]]
git-tree-sha1 = "8c4920235f6c561e401dfe569beb8b924adad003"
uuid = "8be319e6-bccf-4806-a6f7-6fae938471bc"
version = "0.5.0"
"""

# ╔═╡ Cell order:
# ╠═fea40d22-c40a-468d-b74c-fe1a0e6b505d
# ╠═87be8544-979a-11ee-05ab-f78f386a848c
# ╠═7eb57748-3ed4-4fc4-93d1-84ceb7adbfaf
# ╠═57614da3-3db1-4822-a630-7b6275853b63
# ╠═2d974ba2-2110-40cd-8c72-0a33123d5646
# ╠═f2cf036f-e512-4517-89b8-402b40e39c71
# ╠═ef805a41-e34c-4a0f-815a-96b7df45751c
# ╠═ba765d01-00e3-4b2e-9e90-446775dbeb67
# ╠═fa2941f2-9c6a-4814-a0f4-242bc8c41914
# ╟─00000000-0000-0000-0000-000000000001
# ╟─00000000-0000-0000-0000-000000000002
