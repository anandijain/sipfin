;pwd
;cd ..
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
fix_times(df) = Date.(ndf.date_time, dtfmt)

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
