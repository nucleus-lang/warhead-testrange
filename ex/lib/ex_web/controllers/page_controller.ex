defmodule ExWeb.PageController do
  use ExWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end
