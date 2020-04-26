using Plots, CSV, DataFrames, Glob, Dates
using Statistics, StatsBase, Dates
using DelimitedFiles 

;pwd
;cd ..

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

norm_arr(a::AbstractArray) = (a .- mean(a)) ./ std(a)
norm_mat(m::AbstractMatrix) = hcat(map(a -> (a .- mean(a)) ./ std(a), eachcol(m))...)
norm_df(df::AbstractDataFrame) = DataFrame(norm_mat(Matrix(df)), names(df))


# df = CSV.read("../intraday_inner.csv")
# pv = df[:, 3:end]
# df = df[:, 2:end]
ndf = norm_df(pv)
# df = dfs[1]
dtfmt = DateFormat("Y-m-d H:M:S+H:S")
ndf.date_time = Date.(ndf.date_time, dtfmt)

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


df[:, r":US_$(yr)_$(month)_$(day).csv"]
df[:, r"h.*.:X"]
df[:, r"h.*.:F"]

ticks = CSV.read("./data/sp500tickers.txt", header=false)
slugs = CSV.read("./sa.csv")

funds = CSV.read("/home/sippycups/sipfin/mfundslist.txt", delim="|")[1:end-1, :]

to_plot = intersect(lowercase.(ticks.Column1), slugs.slug)

df_col_to_txt("ndaq_funds.txt", funds, Symbol("Fund Symbol"))

reg = r"data\/(.*):"


rename!(x -> Symbol(replace(string(x), " "=>"_")), df)
