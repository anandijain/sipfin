module SipFin

using Plots, Dates
using CSV, DataFrames, Glob
using Statistics, StatsBase, Dates, LinearAlgebra, DelimitedFiles, Base
#= 


=#
norm_arr(a::AbstractArray) = (a .- mean(a)) ./ std(a)
norm_mat(m::AbstractMatrix) = hcat(norm_arr.(eachcol(m))...)
# todo, ignore string columns
norm_df(df::AbstractDataFrame) = DataFrame(norm_mat(Matrix(df)), names(df))
cor_df(df::AbstractDataFrame) = DataFrame(cor(Matrix(df)), names(df))

# tmp = hcat(map(x->pdf.(fit(Normal,x[1]), x[2]), zip([te.c_AAPL, te.c_TSLA], [tr.c_AAPL, tr.c_TSLA]))...)

mavg(vec,n) = [sum(@view vec[i:(i + n - 1)]) / n for i in 1:(length(vec) - (n - 1))]
# todo, also optim w dp instead of recomputing each in loop. mavg(vec,ns::Array{Int, 1}) = [[
fib(n) = ([1 1 ; 1 0]^n)[1, 1]
masps(df::AbstractDataFrame, ns) = by(df, :symbol, p=:x=>x->garbo_plot_mas(x, ns))
function moreg(df)
       ps = []
       for sdf in groupby(df, :symbol)
       p = garbo_plot_mas(sdf.x, 1:2:20)
       plot!(p, title="$(sdf.symbol[1])")
       push!(ps, p)
       end
       ps
       end

function garbo_plot_mas(v, ns) 
	mas = [mavg(v, n) for n in ns]
	p = plot()
	for (n, ma) in zip(ns, mas)
		plot!(ma, label="$(n) mavg", size=(1600, 1600))
	end
	plot(p)
end

#for t in unique(amtbysec.symbol)
#       tmpdf = amtbysec[amtbysec.symbol .== t, :]
#       tmpj = join(spy, tmpdf, on=:t, makeunique=true)
#       println("spy/$(t) amt/sec cor $(cor(tmpj.amt, tmpj.amt_1))")
#       end
#
#jj = join(spy, amtbysec[amtbysec.symbol .== "goog", :], on=:t, makeunique=true)
# diff(df) = df[2:end, :] .- df[1:end - 1, :]
diff_arr(arr::Array) = sum(abs.(arr[2:end] .- arr[1:end - 1]))

cor_df(df::AbstractDataFrame)::AbstractDataFrame = DataFrame(cor(Matrix(df)), names(df))
re_cols(dfs::Array{DataFrame,1}, re::Regex) = map(df->df[:, re], dfs)

# used w nasdaq_o2
# takes a dataframe dictionary and returns the len of each df value
sizes(d::Dict{String,DataFrame})::DataFrame = sort(DataFrame(ticker = collect(keys(d)), nrows = map(x->x[1], size.(collect(values(d))))), :nrows, rev = true)
sizes(dfs::Array{AbstractDataFrame,1})::DataFrame = sort(DataFrame(ticker = map(x->names(x)[2], dfs), nrows = map(x->size(x)[1], dfs)), :nrows, rev = true)

feichanghao(glob_pat)::DataFrame = vcat(collect(values(add_tickers(df_dict(glob_pat))))...)

rep_rm(s::String, rmstr::String)::String = replace(s, rmstr => "")
to_num(s::String)::Float64 = parse(Float64, rep_rm(s, ",")) 

# obviously dangerous, TODO: FIX
# pushed problem to upstream to_csv serializer in rust on may 15th 
# TODO deprecate 
usd_to_float(s::String)::Float64 = parse(Float64, rep_rm(rep_rm(s, "\$"), ","))
usd_col_to_float(df::DataFrame, col::Symbol)::Array{Float64,1} = usd_to_float.(df[:, col])

spreads(df::AbstractDataFrame)::AbstractDataFrame = by(df, :symbol, spread = :x => x->maximum(x) .- minimum(x))
spreads(df::AbstractDataFrame, col::Symbol)::AbstractDataFrame = by(df, col, spread = :x => x->maximum(x) .- minimum(x))

fits(df::AbstractDataFrame) = by(df, :symbol, xfit=:x=>x->fit(Normal,x), vfit=:v=>x-> fit(Normal, x))
fits(df::AbstractDataFrame, col::Union{Symbol, Array{Symbol, 1}}) = by(df, :symbol, xfit=:x=>x->fit(Normal,x), vfit=:v=>x-> fit(Normal, x))


charts_df(df::AbstractDataFrame; size::Tuple = (1600, 1600))::AbstractDataFrame = by(df, :symbol, p = (:t, :x) => x->plot(x.t, x.x, size = size))
charts_df(df::AbstractDataFrame, col::Symbol; size::Tuple = (1600, 1600))::AbstractDataFrame = by(df, col, p = (:t, :x) => x->plot(x.t, x.x, size = size))
save_charts(charts_df::DataFrame; sfx::String="chart") = map(x->savefig(x[2], "$(charts_df.symbol[x[1]])_$(sfx).png"), enumerate(charts_df.p)) 
gen_anims(rt_df::AbstractDataFrame; size=(1200,1200))::AbstractDataFrame = by(rt_df, :symbol, p = (:t, :x, :v) => x->gen_anim(Array(x.t), Array(x,x), Array(x.v)))

function df_col_to_txt(df::AbstractDataFrame, s::Symbol, fn::String)
    open(fn, "w") do io
        writedlm(io, df[:, s], "\n")
    end
end

function str_arr_to_txt(fn::String, arr::Array{String,1})
    open(fn, "w") do io
        writedlm(io, arr, "\n")
    end
end

function change_sep(fn) 
    df = CSV.read("$fn.txt", delim = "|")
    rename!(x->Symbol(replace(string(x), " " => "_")), df)
    CSV.write("$fn.csv", df)
    return df
end


function plot_fibs(arr::AbstractArray, range = 5:10)
    plot([mavg(arr, fib(i)) for i in range])
end


function df_dict(glob_pat)::Dict{String,DataFrame}
    fns = glob(glob_pat)
    Dict(zip(map(x->split(x, "_")[1], fns), CSV.read.(fns)))
end

function add_tickers(d::Dict{String,DataFrame})::Dict{String,DataFrame}
    for (k, v) in d
        v[!, :symbol] .= k
    end
    d
end

function get_dfs(glob_pat)::Array{DataFrame,1}
    CSV.read.(glob(glob_pat))
end


function quick(dfs = get_dfs("./data/*7d*.csv"), re::Regex = r"(c_*)")::AbstractDataFrame # , join_n::Integer = 200, filt_n::Integer = 5000)::AbstractDataFrame
    ts = map(df->df[:, :t], dfs)
    cols = re_cols(dfs, re)
    catted = hcat.(ts, cols)
    # filt = filter(x->size(x)[1] > filt_n, catted) # history size 5000 or greater
    join(catted..., on = :x1)
end


function df_from_str(s::String)
    fn = glob("./data/$(s)_yf7d*.csv")[1]
    df = CSV.read(fn)
end

function garbo_info(p::Array{String,1})
    df = vcat(CSV.read.(readdir())...)
    df.last_sale_price = usd_col_to_float(df, :last_sale_price)
    rn = df[occursin.("May 14", df.last_trade_timestamp), :]
    groupby(sort(rn[in.(rn.symbol, Ref(p)), :], :symbol), :symbol)

end

get_rts()::DataFrame = SipFin.parse_rt(vcat(CSV.read.(glob("**.csv", homedir() * "/D/nasdaq_o2/rt/")))...) 

function parse_rt(df::AbstractDataFrame; to_unixtime::Bool = true)::DataFrame
    df[!, :t] = to_unixtime ? datetime2unix.(today() .+ df[:, :t]) : today() .+ df[:, :t]
    df[!, :amt] = df[:, :x] .* df[:, :v]
    sort!(df, :t)
    df
end

# used to clean the insiders data
function parse_insiders(df) 
    df.last_price = usd_col_to_float(df, :last_price)
    df.shares_traded = parse.(Int, replace.(df.shares_traded, "," => ""))
    df.shares_held =  parse.(Int, replace.(df.shares_held, "," => ""))
    dtfmt = "m/d/y"
    df.last_date = Date.(df.last_date, dtfmt)
end


function summarize_rt(df::DataFrame)::DataFrame
    spreads = by(df, :symbol, xmax = :x => maximum, xmin = :x => minimum)
    summary = by(df, :symbol, nrows = nrow, amt_sum = :amt => sum)
    delts = sort(vcat(map(x->diff_arr(x.x), gdf)...), :x1)
    summary = join([amts, spreads, nrows, delts]..., on = :symbol, makeunique = true)
    # by(df, :symbol, describe)
    # @. sort(by(df, :v, nrow), [:x1, :v], rev=(true, false))
    # by(df, :symbol, tdelt = :t=> x-> maximum(x) - minimum(x))

end

# super fucking slow
function gen_anim(t, x, v)::Animation
    plt = plot3d(
        1,
        xlim = (minimum(t), maximum(t)),
        ylim = (minimum(x), maximum(x)),
        zlim = (minimum(v), maximum(v)),
        title =  "time, price, volume",
        marker = 2,
        size=(1200, 1200)
    )
    anim = Animation()

    for i=1:10:length(t)
        xi, yi, zi = t[i], x[i], v[i]
        push!(plt, (xi, yi, zi))
        frame(anim)
    end
    anim
end

function load(fn)
	df = CSV.read(fn)
	gdf = [DataFrame(x) for x in groupby(df, :symbol)]
	j = join(gdf[1:50]..., on=:t)
	os = j[:, r"o_*"]
	cs = j[:, r"c_*"]

#function frames(dfs::Array{DataFrame,1}) 
#	for df in dfs
#		by(df, :symbol,
end # module
