using Plots
using CSV, DataFrames, Glob
using Statistics, StatsBase, Dates, LinearAlgebra
using DelimitedFiles 
using Base

now = Dates.today()
yr = string(year(now))
month = string(Dates.month(now))
day = string(Dates.day(now))
regex_d = Regex(":US_$(yr)_$(month)_$(day).csv")


stock_fns = Glob.glob("*:US_$(yr)_$(month)_$(day).csv*", "./data/")
com_fns = Glob.glob("*:F_$(yr)_$(month)_$(day).csv*", "./data/")
cur_fns = Glob.glob("*:X_$(yr)_$(month)_$(day).csv*", "./data/")

USs = CSV.read.(stock_fns)
Fs = CSV.read.(com_fns)
Xs = CSV.read.(cur_fns)


dtfmt = DateFormat("Y-m-d H:M:S+H:S")
fix_times(df) = Date.(df.date_time, dtfmt)

function plot_ndf(ndf, df) 
    colnames = names(ndf)
    colnames = names(df)[2:end]
    ndf.date_time

    plot(ndf.date_time, [df.AAPL_price, df.AAPL_volume])

    df = join(dfs..., on=:t, makeunique=true)

    desc = sort(describe(df), :nmissing)

    for cn in colnames
        display(plot(df.date_time, df[:, cn], label=cn))
        print(cn)
    end
end

df[:, r":US_$(yr)_$(month)_$(day).csv"]
df[:, r"h.*.:X"]
df[:, r"h.*.:F"]

ticks = CSV.read("./data/sp500tickers.txt", header=false)
slugs = CSV.read("./sa.csv")
funds = CSV.read("/home/sippycups/sipfin/mfundslist.txt", delim="|")[1:end-1, :]
to_plot = intersect(lowercase.(ticks.Column1), slugs.slug)
Utils.df_col_to_txt("ndaq_funds.txt", funds, Symbol("Fund Symbol"))

desc = describe(joined)
descc = describe(joinedc)
cols = desc[desc.nmissing .== 1, :variable]
colsc = descc[descc.nmissing .== 1, :variable]
ss = dropmissing(joined[:, cols])
avg_vols = mean.(eachcol(ss))
scaled = avg_vols / minimum(avg_vols)
ssc = dropmissing(joinedc[:, colsc])

joinedc = quick(glob_pat, re2, join_n, filt_n)
function drop_sort()
	fn = glob("nasdaq*.csv", "../finox/")[end]
	df = sort(dropmissing(CSV.read(fn)), :percentage_change, rev=true)
	CSV.write(fn, df)
end
    

## todo function that composes 
function corr_5(dfs)
    m = join(rand(dfs, 5)..., on = :t)
    mat = Matrix(m)[:, 2:5:end]
    labs = names(m)[2:5:end]    
    corrplot(mat, label = labs)
end

function sa_yf()
    # we're assuming we just ran finox w cargo run,
    now = Dates.today()
    yr = string(year(now))
    month = string(Dates.month(now))
    day = string(Dates.day(now))
    # fns = Glob.glob("*:US_$(yr)_$(month)_$(day).csv", "./data/")
    sa = CSV.read("./ref_data/sa.csv")
    sa_slugs = uppercase.(unique(dropmissing(sa, :slug).slug))
    # reg = r"data\/(.*):"
    # matches = match.(reg, fns)
    # yf_slugs = map(x -> x.captures[1], matches)
    yf_slugs = readlines("./ref_data/tickers.txt")
    intersect_ticks = intersect(sa_slugs, yf_slugs)
    println(intersect_ticks)
    str_arr_to_txt("./ref_data/intersect_sa_yf.txt", intersect_ticks)
end


function sec13f_fix()
    for fn in Glob.glob("./ref_data/rentec/*.csv")
        header = Symbol.(["nameOfIssuer",
        "titleOfClass",
        "cusip",
        "value",
        "sshPrnamt",
        "sshPrnamtType",
        "investmentDiscretion",
        "otherManager",
        "Sole",
        "Shared",
        "None"])
        df = DataFrame(permutedims(Matrix(CSV.read(fn, header = false))), header)
        print(df)
        CSV.write(fn, df)
    end
end