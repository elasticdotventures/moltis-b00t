#!/usr/bin/env ruby
# frozen_string_literal: true

require "date"

Repo = Data.define(:name, :path, :start_date)

AUTHOR = "Fabien Penso"
WEEKS = 14

repos = [
  Repo.new("Moltis", File.expand_path("../..", __dir__), Date.parse("2026-02-25")),
  Repo.new(
    "Constellations",
    "/Users/penso/code/constellations/constellations-indexer",
    Date.parse("2022-07-01"),
  ),
]

def aggregate(repo)
  data = Array.new(WEEKS) { { commits: 0, additions: 0, deletions: 0 } }
  until_date = repo.start_date + ((WEEKS * 7) - 1)
  command = [
    "git",
    "log",
    "--numstat",
    "--date=short",
    "--pretty=format:@@@%ad%x09%an",
    "--since=#{repo.start_date}",
    "--until=#{until_date} 23:59:59",
  ]

  date = nil
  author = nil

  IO.popen(command, chdir: repo.path) do |io|
    io.each_line do |line|
      line = line.chomp

      if line.start_with?("@@@")
        date_text, author = line[3..].split("\t", 2)
        date = Date.parse(date_text)

        week = ((date - repo.start_date).to_i / 7)
        data[week][:commits] += 1 if author == AUTHOR && week.between?(0, WEEKS - 1)
        next
      end

      next unless author == AUTHOR

      additions_text, deletions_text = line.split("\t", 3)
      next unless additions_text&.match?(/\A\d+\z/) && deletions_text&.match?(/\A\d+\z/)

      week = ((date - repo.start_date).to_i / 7)
      next unless week.between?(0, WEEKS - 1)

      data[week][:additions] += additions_text.to_i
      data[week][:deletions] += deletions_text.to_i
    end
  end

  data
end

def lifetime(repo)
  command = [
    "git",
    "log",
    "--numstat",
    "--date=short",
    "--pretty=format:@@@%ad%x09%an",
  ]

  stats = { commits: 0, additions: 0, deletions: 0, dates: [] }
  author = nil

  IO.popen(command, chdir: repo.path) do |io|
    io.each_line do |line|
      line = line.chomp

      if line.start_with?("@@@")
        date_text, author = line[3..].split("\t", 2)

        if author == AUTHOR
          stats[:commits] += 1
          stats[:dates] << Date.parse(date_text)
        end

        next
      end

      next unless author == AUTHOR

      additions_text, deletions_text = line.split("\t", 3)
      next unless additions_text&.match?(/\A\d+\z/) && deletions_text&.match?(/\A\d+\z/)

      stats[:additions] += additions_text.to_i
      stats[:deletions] += deletions_text.to_i
    end
  end

  first_date = stats[:dates].min
  last_date = stats[:dates].max
  days = (last_date - first_date).to_i + 1

  stats.merge(
    first_date:,
    last_date:,
    days:,
    net: stats[:additions] - stats[:deletions],
    additions_per_day: (stats[:additions].to_f / days).round,
    net_per_day: ((stats[:additions] - stats[:deletions]).to_f / days).round,
  )
end

puts "project,week,start_date,commits,additions,deletions,net"

repos.each do |repo|
  aggregate(repo).each_with_index do |week, index|
    puts [
      repo.name,
      index + 1,
      repo.start_date + (index * 7),
      week[:commits],
      week[:additions],
      week[:deletions],
      week[:additions] - week[:deletions],
    ].join(",")
  end
end

puts
puts "project,first_date,last_date,days,commits,additions,deletions,net,additions_per_day,net_per_day"

repos.each do |repo|
  stats = lifetime(repo)
  puts [
    repo.name,
    stats[:first_date],
    stats[:last_date],
    stats[:days],
    stats[:commits],
    stats[:additions],
    stats[:deletions],
    stats[:net],
    stats[:additions_per_day],
    stats[:net_per_day],
  ].join(",")
end
