using CSV, DataFrames, LinearAlgebra, Statistics, Plots, Distributions, GLM
using DifferentialEquations, Flux, Optim, DiffEqFlux, DiffEqSensitivity

folder = abspath("/home/sippycups/D/sipfin/data/")
fn = "yf_currencies_1590028085.csv"
df = CSV.read("$(folder)$(fn)")
gdf = [DataFrame(x) for x in groupby(df, :symbol)]


cat_symbol(df) = rename(df[:, :], map(x-> replace("$(x)_$(lowercase(df.symbol[1]))", "=x"=>""), names(df)))

garbo_fix(df) = rename(df, names(df)[2]=>:t)
gdf = garbo_fix.(cat_symbol.(gdf))

tmpm = map(x-> x[:, 2:end], gdf)

n = 30
j = join(tmpm[1:n]..., on=:t)
os = j[:, r"o_*"]
os = rename(hcat(j.t, os), :x1=>:t)
os = dropmissing(os)
t = os.t .- os.t[1]

lm1 = fit(LinearModel, @formula(o_usdcny ~ o_eurcny), os)
plot(t, os.o_usdcny, label="o_usdcny")
plot!(t, os.o_eurcny, label="o_eurcny")
plot!(t, predict(lm1), label="lm pred")

jm = Matrix(os[:, 2:end])
# more basic ops
col = jm[:, 1]
μ, σ = mean(col), std(col)

norm_arr(xs) = map(x-> (x - μ) / σ, xs)
norm_arr(col)

norm_mat(m) = hcat(norm_arr.(eachcol(m))...)
nm = norm_mat(jm)

cm = cor(jm)
F = eigen(cm)
U, S, V = svd(cm)


cm2 = cor(nm)
F2 = eigen(cm2)
U2, S2, V2 = svd(cm2)

F.values ≈ F2.values
S ≈ S2

heatmap(abs2.(cm2 - cm))
collect(enumerate(names(os[:, 2:end])))

U, S, V = svd(jm)
U2, S2, V2 = svd(nm)


o = U * Diagonal(S) * V 
jm ≈ o
err = abs2.(jm .- o)
heatmap(err)


# counts = sort(by(df, :symbol, nrow), :x1, rev=true)
# ss = df[in.(df.symbol, Ref(counts.symbol[1:50])), :]


function lotka_volterra!(du, u, p, t)
  x, y = u
  α, β, δ, γ = p
  du[1] = dx = α*x - β*x*y
  du[2] = dy = -δ*y + γ*x*y
end

function lotka_volterra_noise!(du, u, p, t)
  du[1] = 0.1u[1]
  du[2] = 0.1u[2]
end
u0 = jm[1, :]
M = jm[2:end, :]'
t = 
tspan = extrema(t)
p = [2.2, 1.0, 2.0, 0.4]
prob_sde = SDEProblem(lotka_volterra!, lotka_volterra_noise!, u0, tspan)


function predict_sde(p)
  return Array(concrete_solve(prob_sde, SOSRI(), u0, p,
               sensealg = ForwardDiffSensitivity(), saveat = t))
end

loss_sde(p) = sum(abs2, x-1 for x in predict_sde(p))