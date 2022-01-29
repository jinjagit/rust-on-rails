Rails.application.routes.draw do
  resources :tests, path: '/', only: [:index, :create]
end
