using CSV, DataFrames, Glob


function drop_sort()
	fn = glob("nasdaq*.csv", "../finox/")[end]
	df = sort(dropmissing(CSV.read(fn)), :percentage_change, rev=true)
	CSV.write(fn, df)
end
	

drop_sort()
