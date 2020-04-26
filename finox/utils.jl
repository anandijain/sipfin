module Utils
using Plots, Dates
using CSV, DataFrames, Glob
using DelimitedFiles 

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

function df_col_to_txt(fn::String, df::AbstractDataFrame, s::Symbol)
    open(fn, "w") do io
        writedlm(io, df[:, s], "\n")
    end
end

function str_arr_to_txt(fn::String, arr::Array{String, 1})
    open(fn, "w") do io
        writedlm(io, arr, "\n")
    end
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
        df = DataFrame(permutedims(Matrix(CSV.read(fn, header=false))), header)
        print(df)
        CSV.write(fn, df)
    end
end


# sa_yf()
# sec13f_fix()

function change_sep(fn) 
    df = CSV.read("$fn.txt", delim="|")
    rename!(x -> Symbol(replace(string(x), " "=>"_")), df)
    CSV.write("$fn.csv", df)
    return df
end


end